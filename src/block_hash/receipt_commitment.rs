use starknet_types_core::felt::Felt;
use starknet_types_core::hash::StarkHash;

use crate::block::{GasPrice, GasPricePerToken};
use crate::core::ReceiptCommitment;
use crate::crypto::patricia_hash::calculate_root;
use crate::crypto::utils::HashChain;
use crate::hash::starknet_keccak_hash;
use crate::transaction::{
    ExecutionResources, Fee, MessageToL1, TransactionExecutionStatus, TransactionReceipt,
    TransactionVersion,
};

#[cfg(test)]
#[path = "receipt_commitment_test.rs"]
mod receipt_commitment_test;

/// Returns the root of a Patricia tree where each leaf is a receipt hash.
pub fn calculate_receipt_commitment<H: StarkHash>(
    transactions_receipt: &[TransactionReceipt],
    transaction_version: &TransactionVersion,
    l1_data_gas_price_per_token: GasPricePerToken,
    l1_gas_price_per_token: GasPricePerToken,
) -> ReceiptCommitment {
    ReceiptCommitment(calculate_root::<H>(
        transactions_receipt
            .iter()
            .map(|receipt| {
                calculate_receipt_hash(
                    receipt,
                    transaction_version,
                    l1_data_gas_price_per_token,
                    l1_gas_price_per_token,
                )
            })
            .collect(),
    ))
}

// Poseidon(
//    transaction hash, amount of fee paid, hash of messages sent, revert reason,
//    execution resources
// ).
fn calculate_receipt_hash(
    transaction_receipt: &TransactionReceipt,
    transaction_version: &TransactionVersion,
    l1_data_gas_price_per_token: GasPricePerToken,
    l1_gas_price_per_token: GasPricePerToken,
) -> Felt {
    let l1_gas_price = get_price_by_version(l1_gas_price_per_token, transaction_version);
    let l1_data_gas_price = get_price_by_version(l1_data_gas_price_per_token, transaction_version);
    let hash_chain = HashChain::new()
        .chain(&transaction_receipt.transaction_hash)
        .chain(&transaction_receipt.output.actual_fee().0.into())
        .chain(&calculate_messages_sent_hash(transaction_receipt.output.messages_sent()))
        .chain(&get_revert_reason_hash(transaction_receipt.output.execution_status()));
    chain_execution_resources(
        hash_chain,
        transaction_receipt.output.execution_resources(),
        transaction_receipt.output.actual_fee(),
        l1_data_gas_price,
        l1_gas_price,
    )
    .get_poseidon_hash()
}

// Poseidon(
//      num_messages_sent,
//      from_address_0, to_address_0, payload_length_0, payload_0,
//      from_address_1, to_address_1, payload_length_1, payload_1, ...
// ).
fn calculate_messages_sent_hash(messages_sent: &Vec<MessageToL1>) -> Felt {
    let mut messages_hash_chain = HashChain::new().chain(&messages_sent.len().into());
    for message_sent in messages_sent {
        messages_hash_chain = messages_hash_chain
            .chain(&message_sent.from_address)
            .chain(&message_sent.to_address.into())
            .chain_size_and_elements(&message_sent.payload.0);
    }
    messages_hash_chain.get_poseidon_hash()
}

// Returns starknet-keccak of the revert reason ASCII string, or 0 if the transaction succeeded.
fn get_revert_reason_hash(execution_status: &TransactionExecutionStatus) -> Felt {
    match execution_status {
        TransactionExecutionStatus::Succeeded => Felt::ZERO,
        TransactionExecutionStatus::Reverted(reason) => {
            starknet_keccak_hash(reason.revert_reason.as_bytes())
        }
    }
}

// Chains:
// L2 gas consumed (In the current RPC: always 0),
// L1 gas consumed (In the current RPC:
//      L1 gas consumed for calldata + L1 gas consumed for steps and builtins.
//      Calculated as: (actual_fee - actual_l1_data_gas_fee) / l1_gas_price
// L1 data gas consumed (In the current RPC: L1 data gas consumed for blob).
fn chain_execution_resources(
    hash_chain: HashChain,
    execution_resources: &ExecutionResources,
    actual_fee: Fee,
    l1_data_gas_price: GasPrice,
    l1_gas_price: GasPrice,
) -> HashChain {
    let l1_gas_consumed: u128 = (actual_fee.0
        - (l1_data_gas_price.0) * u128::from(execution_resources.da_l1_data_gas_consumed))
        / l1_gas_price.0;
    hash_chain
        .chain(&Felt::ZERO) // L2 gas consumed
        .chain(&l1_gas_consumed.into())
        .chain(&execution_resources.da_l1_data_gas_consumed.into())
}

// TODO(yoav): move this function to transaction.rs and make it public.
fn get_price_by_version(
    price_per_token: GasPricePerToken,
    transaction_version: &TransactionVersion,
) -> GasPrice {
    if transaction_version >= &TransactionVersion::THREE {
        price_per_token.price_in_fri
    } else {
        price_per_token.price_in_wei
    }
}
