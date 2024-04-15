use once_cell::sync::Lazy;
use starknet_types_core::felt::Felt;

use crate::block::BlockNumber;
use crate::core::{calculate_contract_address, ChainId, ContractAddress};
use crate::crypto::utils::HashChain;
use crate::data_availability::DataAvailabilityMode;
use crate::transaction::{
    DeclareTransaction, DeclareTransactionV0V1, DeclareTransactionV2, DeclareTransactionV3,
    DeployAccountTransaction, DeployAccountTransactionV1, DeployAccountTransactionV3,
    DeployTransaction, InvokeTransaction, InvokeTransactionV0, InvokeTransactionV1,
    InvokeTransactionV3, L1HandlerTransaction, Resource, ResourceBounds, ResourceBoundsMapping,
    Tip, Transaction, TransactionHash, TransactionVersion,
};
use crate::StarknetApiError;

type ResourceName = [u8; 7];

const DATA_AVAILABILITY_MODE_BITS: usize = 32;
const L1_GAS: &ResourceName = b"\0L1_GAS";
const L2_GAS: &ResourceName = b"\0L2_GAS";

static DECLARE: Lazy<Felt> =
    Lazy::new(|| ascii_as_felt("declare").expect("ascii_as_felt failed for 'declare'"));
static DEPLOY: Lazy<Felt> =
    Lazy::new(|| ascii_as_felt("deploy").expect("ascii_as_felt failed for 'deploy'"));
static DEPLOY_ACCOUNT: Lazy<Felt> = Lazy::new(|| {
    ascii_as_felt("deploy_account").expect("ascii_as_felt failed for 'deploy_account'")
});
static INVOKE: Lazy<Felt> =
    Lazy::new(|| ascii_as_felt("invoke").expect("ascii_as_felt failed for 'invoke'"));
static L1_HANDLER: Lazy<Felt> =
    Lazy::new(|| ascii_as_felt("l1_handler").expect("ascii_as_felt failed for 'l1_handler'"));
const CONSTRUCTOR_ENTRY_POINT_SELECTOR: Felt =
    Felt::from_hex_unchecked("0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194");

/// Calculates hash of a Starknet transaction.
pub fn get_transaction_hash(
    transaction: &Transaction,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    match transaction {
        Transaction::Declare(declare) => match declare {
            DeclareTransaction::V0(declare_v0) => {
                get_declare_transaction_v0_hash(declare_v0, chain_id, transaction_version)
            }
            DeclareTransaction::V1(declare_v1) => {
                get_declare_transaction_v1_hash(declare_v1, chain_id, transaction_version)
            }
            DeclareTransaction::V2(declare_v2) => {
                get_declare_transaction_v2_hash(declare_v2, chain_id, transaction_version)
            }
            DeclareTransaction::V3(declare_v3) => {
                get_declare_transaction_v3_hash(declare_v3, chain_id, transaction_version)
            }
        },
        Transaction::Deploy(deploy) => {
            get_deploy_transaction_hash(deploy, chain_id, transaction_version)
        }
        Transaction::DeployAccount(deploy_account) => match deploy_account {
            DeployAccountTransaction::V1(deploy_account_v1) => {
                get_deploy_account_transaction_v1_hash(
                    deploy_account_v1,
                    chain_id,
                    transaction_version,
                )
            }
            DeployAccountTransaction::V3(deploy_account_v3) => {
                get_deploy_account_transaction_v3_hash(
                    deploy_account_v3,
                    chain_id,
                    transaction_version,
                )
            }
        },
        Transaction::Invoke(invoke) => match invoke {
            InvokeTransaction::V0(invoke_v0) => {
                get_invoke_transaction_v0_hash(invoke_v0, chain_id, transaction_version)
            }
            InvokeTransaction::V1(invoke_v1) => {
                get_invoke_transaction_v1_hash(invoke_v1, chain_id, transaction_version)
            }
            InvokeTransaction::V3(invoke_v3) => {
                get_invoke_transaction_v3_hash(invoke_v3, chain_id, transaction_version)
            }
        },
        Transaction::L1Handler(l1_handler) => {
            get_l1_handler_transaction_hash(l1_handler, chain_id, transaction_version)
        }
    }
}

// On mainnet, from this block number onwards, there are no deprecated transactions,
// enabling us to validate against a single hash calculation.
const MAINNET_TRANSACTION_HASH_WITH_VERSION: BlockNumber = BlockNumber(1470);

// Calculates a list of deprecated hashes for a transaction.
fn get_deprecated_transaction_hashes(
    chain_id: &ChainId,
    block_number: &BlockNumber,
    transaction: &Transaction,
    transaction_version: &TransactionVersion,
) -> Result<Vec<TransactionHash>, StarknetApiError> {
    Ok(
        if chain_id == &ChainId("SN_MAIN".to_string())
            && block_number > &MAINNET_TRANSACTION_HASH_WITH_VERSION
        {
            vec![]
        } else {
            match transaction {
                Transaction::Declare(_) => vec![],
                Transaction::Deploy(deploy) => {
                    vec![get_deprecated_deploy_transaction_hash(
                        deploy,
                        chain_id,
                        transaction_version,
                    )?]
                }
                Transaction::DeployAccount(_) => vec![],
                Transaction::Invoke(invoke) => match invoke {
                    InvokeTransaction::V0(invoke_v0) => {
                        vec![get_deprecated_invoke_transaction_v0_hash(
                            invoke_v0,
                            chain_id,
                            transaction_version,
                        )?]
                    }
                    InvokeTransaction::V1(_) | InvokeTransaction::V3(_) => vec![],
                },
                Transaction::L1Handler(l1_handler) => get_deprecated_l1_handler_transaction_hashes(
                    l1_handler,
                    chain_id,
                    transaction_version,
                )?,
            }
        },
    )
}

/// Validates the hash of a starknet transaction.
/// For transactions on testnet or those with a low block_number, we validate the
/// transaction hash against all potential historical hash computations. For recent
/// transactions on mainnet, the hash is validated by calculating the precise hash
/// based on the transaction version.
pub fn validate_transaction_hash(
    transaction: &Transaction,
    block_number: &BlockNumber,
    chain_id: &ChainId,
    expected_hash: TransactionHash,
    transaction_version: &TransactionVersion,
) -> Result<bool, StarknetApiError> {
    let mut possible_hashes = get_deprecated_transaction_hashes(
        chain_id,
        block_number,
        transaction,
        transaction_version,
    )?;
    possible_hashes.push(get_transaction_hash(transaction, chain_id, transaction_version)?);
    Ok(possible_hashes.contains(&expected_hash))
}

// TODO: should be part of core::Felt
fn ascii_as_felt(ascii_str: &str) -> Result<Felt, StarknetApiError> {
    Felt::from_hex(hex::encode(ascii_str).as_str())
        .map_err(|_| StarknetApiError::OutOfRange { string: ascii_str.to_string() })
}

// An implementation of the SNIP: https://github.com/EvyatarO/SNIPs/blob/snip-8/SNIPS/snip-8.md
fn get_tip_resource_bounds_hash(
    resource_bounds_mapping: &ResourceBoundsMapping,
    tip: &Tip,
) -> Result<Felt, StarknetApiError> {
    let l1_resource_bounds =
        resource_bounds_mapping.0.get(&Resource::L1Gas).expect("Missing l1 resource");
    let l1_resource = get_concat_resource(l1_resource_bounds, L1_GAS)?;

    let l2_resource_bounds =
        resource_bounds_mapping.0.get(&Resource::L2Gas).expect("Missing l2 resource");
    let l2_resource = get_concat_resource(l2_resource_bounds, L2_GAS)?;

    Ok(HashChain::new()
        .chain(&tip.0.into())
        .chain(&l1_resource)
        .chain(&l2_resource)
        .get_poseidon_hash())
}

// Receives resource_bounds and resource_name and returns:
// [0 | resource_name (56 bit) | max_amount (64 bit) | max_price_per_unit (128 bit)].
// An implementation of the SNIP: https://github.com/EvyatarO/SNIPs/blob/snip-8/SNIPS/snip-8.md.
fn get_concat_resource(
    resource_bounds: &ResourceBounds,
    resource_name: &ResourceName,
) -> Result<Felt, StarknetApiError> {
    let max_amount = resource_bounds.max_amount.to_be_bytes();
    let max_price = resource_bounds.max_price_per_unit.to_be_bytes();
    let concat_bytes =
        [[0_u8].as_slice(), resource_name.as_slice(), max_amount.as_slice(), max_price.as_slice()]
            .concat();
    Ok(Felt::from_bytes_be(&concat_bytes.try_into().expect("Expect 32 bytes")))
}

// Receives nonce_mode and fee_mode and returns:
// [0...0 (192 bit) | nonce_mode (32 bit) | fee_mode (32 bit)].
// An implementation of the SNIP: https://github.com/EvyatarO/SNIPs/blob/snip-8/SNIPS/snip-8.md.
fn concat_data_availability_mode(
    nonce_mode: &DataAvailabilityMode,
    fee_mode: &DataAvailabilityMode,
) -> Felt {
    (data_availability_mode_index(fee_mode)
        + (data_availability_mode_index(nonce_mode) << DATA_AVAILABILITY_MODE_BITS))
        .into()
}

fn data_availability_mode_index(mode: &DataAvailabilityMode) -> u64 {
    match mode {
        DataAvailabilityMode::L1 => 0,
        DataAvailabilityMode::L2 => 1,
    }
}

pub(crate) fn get_deploy_transaction_hash(
    transaction: &DeployTransaction,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    get_common_deploy_transaction_hash(transaction, chain_id, false, transaction_version)
}

fn get_deprecated_deploy_transaction_hash(
    transaction: &DeployTransaction,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    get_common_deploy_transaction_hash(transaction, chain_id, true, transaction_version)
}

fn get_common_deploy_transaction_hash(
    transaction: &DeployTransaction,
    chain_id: &ChainId,
    is_deprecated: bool,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    let contract_address = calculate_contract_address(
        transaction.contract_address_salt,
        transaction.class_hash,
        &transaction.constructor_calldata,
        ContractAddress::from(0_u8),
    )?;

    Ok(TransactionHash(
        HashChain::new()
        .chain(&DEPLOY)
        .chain_if_fn(|| {
            if !is_deprecated {
                Some(transaction_version.0)
            } else {
                None
            }
        })
        .chain(contract_address.0.key())
        .chain(&CONSTRUCTOR_ENTRY_POINT_SELECTOR)
        .chain(
            &HashChain::new()
                .chain_iter(transaction.constructor_calldata.0.iter())
                .get_pedersen_hash(),
        )
         // No fee in deploy transaction.
        .chain_if_fn(|| {
            if !is_deprecated {
                Some(Felt::ZERO)
            } else {
                None
            }
        })
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_invoke_transaction_v0_hash(
    transaction: &InvokeTransactionV0,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    get_common_invoke_transaction_v0_hash(transaction, chain_id, false, transaction_version)
}

fn get_deprecated_invoke_transaction_v0_hash(
    transaction: &InvokeTransactionV0,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    get_common_invoke_transaction_v0_hash(transaction, chain_id, true, transaction_version)
}

fn get_common_invoke_transaction_v0_hash(
    transaction: &InvokeTransactionV0,
    chain_id: &ChainId,
    is_deprecated: bool,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
            .chain(&INVOKE)
            .chain_if_fn(|| if !is_deprecated { Some(transaction_version.0) } else { None })
            .chain(transaction.contract_address.0.key())
            .chain(&transaction.entry_point_selector.0)
            .chain(&HashChain::new().chain_iter(transaction.calldata.0.iter()).get_pedersen_hash())
            .chain_if_fn(|| if !is_deprecated { Some(transaction.max_fee.0.into()) } else { None })
            .chain(&ascii_as_felt(chain_id.0.as_str())?)
            .get_pedersen_hash(),
    ))
}

pub(crate) fn get_invoke_transaction_v1_hash(
    transaction: &InvokeTransactionV1,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
        .chain(&INVOKE)
        .chain(&transaction_version.0)
        .chain(transaction.sender_address.0.key())
        .chain(&Felt::ZERO) // No entry point selector in invoke transaction.
        .chain(&HashChain::new().chain_iter(transaction.calldata.0.iter()).get_pedersen_hash())
        .chain(&transaction.max_fee.0.into())
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain(&transaction.nonce.0)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_invoke_transaction_v3_hash(
    transaction: &InvokeTransactionV3,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    let tip_resource_bounds_hash =
        get_tip_resource_bounds_hash(&transaction.resource_bounds, &transaction.tip)?;
    let paymaster_data_hash =
        HashChain::new().chain_iter(transaction.paymaster_data.0.iter()).get_poseidon_hash();
    let data_availability_mode = concat_data_availability_mode(
        &transaction.nonce_data_availability_mode,
        &transaction.fee_data_availability_mode,
    );
    let account_deployment_data_hash = HashChain::new()
        .chain_iter(transaction.account_deployment_data.0.iter())
        .get_poseidon_hash();
    let calldata_hash =
        HashChain::new().chain_iter(transaction.calldata.0.iter()).get_poseidon_hash();

    Ok(TransactionHash(
        HashChain::new()
            .chain(&INVOKE)
            .chain(&transaction_version.0)
            .chain(transaction.sender_address.0.key())
            .chain(&tip_resource_bounds_hash)
            .chain(&paymaster_data_hash)
            .chain(&ascii_as_felt(chain_id.0.as_str())?)
            .chain(&transaction.nonce.0)
            .chain(&data_availability_mode)
            .chain(&account_deployment_data_hash)
            .chain(&calldata_hash)
            .get_poseidon_hash(),
    ))
}

#[derive(PartialEq, PartialOrd)]
enum L1HandlerVersions {
    AsInvoke,
    V0Deprecated,
    V0,
}

pub(crate) fn get_l1_handler_transaction_hash(
    transaction: &L1HandlerTransaction,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    get_common_l1_handler_transaction_hash(
        transaction,
        chain_id,
        L1HandlerVersions::V0,
        transaction_version,
    )
}

fn get_deprecated_l1_handler_transaction_hashes(
    transaction: &L1HandlerTransaction,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<Vec<TransactionHash>, StarknetApiError> {
    Ok(vec![
        get_common_l1_handler_transaction_hash(
            transaction,
            chain_id,
            L1HandlerVersions::AsInvoke,
            transaction_version,
        )?,
        get_common_l1_handler_transaction_hash(
            transaction,
            chain_id,
            L1HandlerVersions::V0Deprecated,
            transaction_version,
        )?,
    ])
}

fn get_common_l1_handler_transaction_hash(
    transaction: &L1HandlerTransaction,
    chain_id: &ChainId,
    version: L1HandlerVersions,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
        .chain_if_fn(|| {
            if version == L1HandlerVersions::AsInvoke {
                Some(*INVOKE)
            } else {
                Some(*L1_HANDLER)
            }
        })
        .chain_if_fn(|| {
            if version > L1HandlerVersions::V0Deprecated {
                Some(transaction_version.0)
            } else {
                None
            }
        })
        .chain(transaction.contract_address.0.key())
        .chain(&transaction.entry_point_selector.0)
        .chain(&HashChain::new().chain_iter(transaction.calldata.0.iter()).get_pedersen_hash())
        // No fee in l1 handler transaction.
        .chain_if_fn(|| {
            if version > L1HandlerVersions::V0Deprecated {
                Some(Felt::ZERO)
            } else {
                None
            }
        })
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain_if_fn(|| {
            if version > L1HandlerVersions::AsInvoke {
                Some(transaction.nonce.0)
            } else {
                None
            }
        })
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_declare_transaction_v0_hash(
    transaction: &DeclareTransactionV0V1,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
        .chain(&DECLARE)
        .chain(&transaction_version.0)
        .chain(transaction.sender_address.0.key())
        .chain(&Felt::ZERO) // No entry point selector in declare transaction.
        .chain(&HashChain::new().get_pedersen_hash())
        .chain(&transaction.max_fee.0.into())
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain(&transaction.class_hash.0)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_declare_transaction_v1_hash(
    transaction: &DeclareTransactionV0V1,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
        .chain(&DECLARE)
        .chain(&transaction_version.0)
        .chain(transaction.sender_address.0.key())
        .chain(&Felt::ZERO) // No entry point selector in declare transaction.
        .chain(&HashChain::new().chain(&transaction.class_hash.0).get_pedersen_hash())
        .chain(&transaction.max_fee.0.into())
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain(&transaction.nonce.0)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_declare_transaction_v2_hash(
    transaction: &DeclareTransactionV2,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    Ok(TransactionHash(
        HashChain::new()
        .chain(&DECLARE)
        .chain(&transaction_version.0)
        .chain(transaction.sender_address.0.key())
        .chain(&Felt::ZERO) // No entry point selector in declare transaction.
        .chain(&HashChain::new().chain(&transaction.class_hash.0).get_pedersen_hash())
        .chain(&transaction.max_fee.0.into())
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain(&transaction.nonce.0)
        .chain(&transaction.compiled_class_hash.0)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_declare_transaction_v3_hash(
    transaction: &DeclareTransactionV3,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    let tip_resource_bounds_hash =
        get_tip_resource_bounds_hash(&transaction.resource_bounds, &transaction.tip)?;
    let paymaster_data_hash =
        HashChain::new().chain_iter(transaction.paymaster_data.0.iter()).get_poseidon_hash();
    let data_availability_mode = concat_data_availability_mode(
        &transaction.nonce_data_availability_mode,
        &transaction.fee_data_availability_mode,
    );
    let account_deployment_data_hash = HashChain::new()
        .chain_iter(transaction.account_deployment_data.0.iter())
        .get_poseidon_hash();

    Ok(TransactionHash(
        HashChain::new()
            .chain(&DECLARE)
            .chain(&transaction_version.0)
            .chain(transaction.sender_address.0.key())
            .chain(&tip_resource_bounds_hash)
            .chain(&paymaster_data_hash)
            .chain(&ascii_as_felt(chain_id.0.as_str())?)
            .chain(&transaction.nonce.0)
            .chain(&data_availability_mode)
            .chain(&account_deployment_data_hash)
            .chain(&transaction.class_hash.0)
            .chain(&transaction.compiled_class_hash.0)
            .get_poseidon_hash(),
    ))
}

pub(crate) fn get_deploy_account_transaction_v1_hash(
    transaction: &DeployAccountTransactionV1,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    let calldata_hash = HashChain::new()
        .chain(&transaction.class_hash.0)
        .chain(&transaction.contract_address_salt.0)
        .chain_iter(transaction.constructor_calldata.0.iter())
        .get_pedersen_hash();

    let contract_address = calculate_contract_address(
        transaction.contract_address_salt,
        transaction.class_hash,
        &transaction.constructor_calldata,
        ContractAddress::from(0_u8),
    )?;

    Ok(TransactionHash(
        HashChain::new()
        .chain(&DEPLOY_ACCOUNT)
        .chain(&transaction_version.0)
        .chain(contract_address.0.key())
        .chain(&Felt::ZERO) // No entry point selector in deploy account transaction.
        .chain(&calldata_hash)
        .chain(&transaction.max_fee.0.into())
        .chain(&ascii_as_felt(chain_id.0.as_str())?)
        .chain(&transaction.nonce.0)
        .get_pedersen_hash(),
    ))
}

pub(crate) fn get_deploy_account_transaction_v3_hash(
    transaction: &DeployAccountTransactionV3,
    chain_id: &ChainId,
    transaction_version: &TransactionVersion,
) -> Result<TransactionHash, StarknetApiError> {
    let contract_address = calculate_contract_address(
        transaction.contract_address_salt,
        transaction.class_hash,
        &transaction.constructor_calldata,
        ContractAddress::from(0_u8),
    )?;
    let tip_resource_bounds_hash =
        get_tip_resource_bounds_hash(&transaction.resource_bounds, &transaction.tip)?;
    let paymaster_data_hash =
        HashChain::new().chain_iter(transaction.paymaster_data.0.iter()).get_poseidon_hash();
    let data_availability_mode = concat_data_availability_mode(
        &transaction.nonce_data_availability_mode,
        &transaction.fee_data_availability_mode,
    );
    let constructor_calldata_hash =
        HashChain::new().chain_iter(transaction.constructor_calldata.0.iter()).get_poseidon_hash();

    Ok(TransactionHash(
        HashChain::new()
            .chain(&DEPLOY_ACCOUNT)
            .chain(&transaction_version.0)
            .chain(contract_address.0.key())
            .chain(&tip_resource_bounds_hash)
            .chain(&paymaster_data_hash)
            .chain(&ascii_as_felt(chain_id.0.as_str())?)
            .chain(&data_availability_mode)
            .chain(&transaction.nonce.0)
            .chain(&constructor_calldata_hash)
            .chain(&transaction.class_hash.0)
            .chain(&transaction.contract_address_salt.0)
            .get_poseidon_hash(),
    ))
}
