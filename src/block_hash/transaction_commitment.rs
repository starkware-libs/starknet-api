use crate::core::TransactionCommitment;
use crate::crypto::patricia_hash::calculate_root;
use crate::crypto::utils::HashChain;
use crate::hash::{HashFunction, StarkFelt};
use crate::transaction::{TransactionHash, TransactionSignature};

#[cfg(test)]
#[path = "transaction_commitment_test.rs"]
mod transaction_commitment_test;

/// The elements used to calculate a leaf in the transactions Patricia tree.
#[derive(Clone)]
pub struct TransactionLeafElement {
    pub transaction_hash: TransactionHash,
    pub transaction_signature: Option<TransactionSignature>,
}

/// Returns the root of a Patricia tree where each leaf is
/// Poseidon(transaction_hash, transaction_signature).
/// The leaf of a transaction types without a signature field is: Poseidon(transaction_hash, 0).
pub fn calculate_transactions_commitment<H: HashFunction>(
    transaction_leaf_elements: &[TransactionLeafElement],
) -> TransactionCommitment {
    let transaction_leaves =
        transaction_leaf_elements.iter().map(calculate_transaction_leaf).collect();
    TransactionCommitment(calculate_root::<H>(transaction_leaves))
}

fn calculate_transaction_leaf(transaction_leaf_elements: &TransactionLeafElement) -> StarkFelt {
    HashChain::new()
        .chain(&transaction_leaf_elements.transaction_hash.0)
        .chain_iter(
            transaction_leaf_elements
                .transaction_signature
                .as_ref()
                .unwrap_or(&TransactionSignature(vec![StarkFelt::ZERO]))
                .0
                .iter(),
        )
        .get_poseidon_hash()
}
