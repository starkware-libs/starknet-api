use std::collections::BTreeMap;
use std::sync::Arc;

use rstest::rstest;
use starknet_types_core::felt::Felt;

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce, PatriciaKey};
use crate::external_transaction::{
    ContractClass, DataAvailabilityMode, ExternalDeclareTransaction, ExternalDeclareTransactionV3,
    ExternalDeployAccountTransaction, ExternalDeployAccountTransactionV3,
    ExternalInvokeTransaction, ExternalInvokeTransactionV3, ExternalTransaction,
};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, Resource, ResourceBounds,
    ResourceBoundsMapping, Tip, TransactionSignature,
};
use crate::hash::StarkHash;
use crate::{contract_address, patricia_key};

fn create_resource_bounds() -> ResourceBoundsMapping {
    let mut map = BTreeMap::new();
    map.insert(Resource::L1Gas, ResourceBounds { max_amount: 100, max_price_per_unit: 12 });
    map.insert(Resource::L2Gas, ResourceBounds { max_amount: 58, max_price_per_unit: 31 });
    ResourceBoundsMapping(map)
}

fn create_declare_v3() -> ExternalDeclareTransaction {
    ExternalDeclareTransaction::V3(ExternalDeclareTransactionV3 {
        contract_class: ContractClass::default(),
        resource_bounds: create_resource_bounds(),
        tip: Tip(1),
        signature: TransactionSignature(vec![Felt::ONE, Felt::TWO]),
        nonce: Nonce(Felt::from_hex_unchecked("0x1")),
        compiled_class_hash: CompiledClassHash(Felt::from_hex_unchecked("0x2")),
        sender_address: contract_address!("0x3"),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L2,
        paymaster_data: PaymasterData(vec![Felt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![Felt::THREE]),
    })
}

fn create_deploy_account_v3() -> ExternalDeployAccountTransaction {
    ExternalDeployAccountTransaction::V3(ExternalDeployAccountTransactionV3 {
        resource_bounds: create_resource_bounds(),
        tip: Tip::default(),
        contract_address_salt: ContractAddressSalt(Felt::from_hex_unchecked("0x23")),
        class_hash: ClassHash(Felt::from_hex_unchecked("0x2")),
        constructor_calldata: Calldata(Arc::new(vec![Felt::ZERO])),
        nonce: Nonce(Felt::from_hex_unchecked("0x60")),
        signature: TransactionSignature(vec![Felt::TWO]),
        nonce_data_availability_mode: DataAvailabilityMode::L2,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![Felt::TWO, Felt::ZERO]),
    })
}

fn create_invoke_v3() -> ExternalInvokeTransaction {
    ExternalInvokeTransaction::V3(ExternalInvokeTransactionV3 {
        resource_bounds: create_resource_bounds(),
        tip: Tip(50),
        calldata: Calldata(Arc::new(vec![
            Felt::from_hex_unchecked("0x2000"),
            Felt::from_hex_unchecked("0x1000"),
        ])),
        sender_address: contract_address!("0x53"),
        nonce: Nonce(Felt::from_hex_unchecked("0x32")),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![Felt::TWO, Felt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![Felt::from_hex_unchecked("0x87")]),
    })
}

#[rstest]
#[case(ExternalTransaction::Declare(create_declare_v3()))]
#[case(ExternalTransaction::DeployAccount(create_deploy_account_v3()))]
#[case(ExternalTransaction::Invoke(create_invoke_v3()))]
fn test_external_transactions(#[case] tx: ExternalTransaction) {
    let serialized = serde_json::to_string(&tx).unwrap();
    let deserialized: ExternalTransaction = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tx, deserialized);
}
