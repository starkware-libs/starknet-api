use crate::block_hash::transaction_commitment::{
    calculate_transaction_leaf, calculate_transactions_commitment, TransactionLeafElements,
};
use crate::core::TransactionCommitment;
use crate::hash::{PoseidonHashCalculator, StarkFelt};
use crate::transaction::{TransactionHash, TransactionSignature};

#[test]
fn test_transaction_leaf_regression() {
    let transaction_hash = TransactionHash(StarkFelt::ONE);
    let transaction_signature = TransactionSignature(vec![StarkFelt::TWO, StarkFelt::THREE]);
    let transaction_leaf_elements =
        TransactionLeafElements { transaction_hash, transaction_signature };

    let expected_leaf =
        StarkFelt::try_from("0x2f0d8840bcf3bc629598d8a6cc80cb7c0d9e52d93dab244bbf9cd0dca0ad082")
            .unwrap();

    assert_eq!(expected_leaf, calculate_transaction_leaf(&transaction_leaf_elements));
}

#[test]
fn test_transactions_commitment_regression() {
    let transaction_hash = TransactionHash(StarkFelt::ONE);
    let transaction_signature = TransactionSignature(vec![StarkFelt::TWO, StarkFelt::THREE]);
    let transaction_leaf_elements =
        TransactionLeafElements { transaction_hash, transaction_signature };

    let expected_root =
        StarkFelt::try_from("0x0282b635972328bd1cfa86496fe920d20bd9440cd78ee8dc90ae2b383d664dcf")
            .unwrap();

    assert_eq!(
        TransactionCommitment(expected_root),
        calculate_transactions_commitment::<PoseidonHashCalculator>(&[
            transaction_leaf_elements.clone(),
            transaction_leaf_elements
        ],)
    );
}
