use starknet_types_core::felt::Felt;
use starknet_types_core::hash::StarkHash;

use crate::core::ReceiptCommitment;
use crate::crypto::patricia_hash::calculate_root;
use crate::crypto::utils::HashChain;
use crate::transaction::{
    ExecutionResources, MessageToL1, TransactionExecutionStatus, TransactionReceipt,
};
use crate::transaction_hash::ascii_as_felt;

#[cfg(test)]
#[path = "receipt_commitment_test.rs"]
mod receipt_commitment_test;

// TODO(yoav): Get these values from versioned constants.
const L1_GAS_PER_STEP: u64 = 1;
const L1_GAS_PER_BUILTIN_INSTANCE: u64 = 1;

/// Returns the root of a Patricia tree where each leaf is a receipt hash.
pub fn calculate_receipt_commitment<H: StarkHash>(
    transactions_receipt: &[TransactionReceipt],
) -> ReceiptCommitment {
    ReceiptCommitment(calculate_root::<H>(
        transactions_receipt.iter().map(calculate_receipt_hash).collect(),
    ))
}

// Poseidon(
//    transaction hash, amount of fee paid, hash of messages sent, revert reason,
//    execution resources
// ).
fn calculate_receipt_hash(transaction_receipt: &TransactionReceipt) -> Felt {
    let hash_chain = HashChain::new()
        .chain(&transaction_receipt.transaction_hash)
        .chain(&transaction_receipt.output.actual_fee().0.into())
        .chain(&calculate_messages_sent_hash(transaction_receipt.output.messages_sent()))
        .chain(&get_revert_reason(transaction_receipt.output.execution_status()));
    chain_execution_resources(hash_chain, transaction_receipt.output.execution_resources())
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
fn get_revert_reason(execution_status: &TransactionExecutionStatus) -> Felt {
    match execution_status {
        TransactionExecutionStatus::Succeeded => Felt::ZERO,
        // TODO(yoav): Wrap the ASCII with a keccak.
        TransactionExecutionStatus::Reverted(reason) => {
            ascii_as_felt(&reason.revert_reason).expect("ascii_as_felt failed for revert reason")
        }
    }
}

// Chains:
// L2 gas consumed (In the current RPC: always 0),
// L1 gas consumed (In the current RPC:
//      L1 gas consumed for calldata + L1 gas consumed for steps and builtins),
// L1 data gas consumed (In the current RPC: L1 data gas consumed for blob).
fn chain_execution_resources(
    hash_chain: HashChain,
    execution_resources: &ExecutionResources,
) -> HashChain {
    let l1_gas_consumed = execution_resources.da_l1_gas_consumed
        + execution_resources.steps * L1_GAS_PER_STEP
        + execution_resources.builtin_instance_counter.values().sum::<u64>()
            * L1_GAS_PER_BUILTIN_INSTANCE;
    hash_chain
        .chain(&Felt::ZERO) // L2 gas consumed
        .chain(&l1_gas_consumed.into())
        .chain(&execution_resources.da_l1_data_gas_consumed.into())
}
