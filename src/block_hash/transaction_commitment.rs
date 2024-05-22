use starknet_types_core::felt::Felt;
use starknet_types_core::hash::StarkHash as CoreStarkHash;

use crate::core::TransactionCommitment;
use crate::crypto::patricia_hash::calculate_root;
use crate::crypto::utils::HashChain;
use crate::transaction::{TransactionHash, TransactionSignature};

#[cfg(test)]
#[path = "transaction_commitment_test.rs"]
mod transaction_commitment_test;

/// The elements used to calculate a leaf in the transactions Patricia tree.
#[derive(Clone)]
pub struct TransactionLeafElements {
    transaction_hash: TransactionHash,
    transaction_signature: TransactionSignature,
}

/// Returns the root of a Patricia tree where each leaf is
/// Poseidon(transaction_hash, transaction_signature).
pub fn calculate_transactions_commitment<H: CoreStarkHash>(
    transaction_leaf_elements: &[TransactionLeafElements],
) -> TransactionCommitment {
    let transaction_leaves =
        transaction_leaf_elements.iter().map(calculate_transaction_leaf).collect();
    TransactionCommitment(calculate_root::<H>(transaction_leaves))
}

fn calculate_transaction_leaf(transaction_leaf_elements: &TransactionLeafElements) -> Felt {
    HashChain::new()
        .chain(&transaction_leaf_elements.transaction_hash.0)
        .chain_iter(transaction_leaf_elements.transaction_signature.0.iter())
        .get_poseidon_hash()
}
