use super::calculate_messages_sent_hash;
use crate::block_hash::receipt_commitment::{
    calculate_receipt_commitment, calculate_receipt_hash, get_revert_reason_hash,
};
use crate::block_hash::test_utils::{generate_message_to_l1, get_transaction_reciept};
use crate::core::ReceiptCommitment;
use crate::hash::{PoseidonHashCalculator, StarkFelt};
use crate::transaction::{RevertedTransactionExecutionStatus, TransactionExecutionStatus};

#[test]
fn test_receipt_hash_regression() {
    let transaction_receipt = get_transaction_reciept();

    let expected_hash =
        StarkFelt::try_from("0x06720e8f1cd4543ae25714f0c79e592d98b16747a92962406ab08b6d46e10fd2")
            .unwrap();
    assert_eq!(calculate_receipt_hash(&transaction_receipt), expected_hash);

    let expected_root = ReceiptCommitment(
        StarkFelt::try_from("0x035481a28b3ea40ddfbc80f3eabc924c9c5edf64f1dd7467d80c5c5290cf48ad")
            .unwrap(),
    );
    assert_eq!(
        calculate_receipt_commitment::<PoseidonHashCalculator>(&[transaction_receipt]),
        expected_root
    );
}

#[test]
fn test_messages_sent_regression() {
    let messages_sent = vec![generate_message_to_l1(0), generate_message_to_l1(1)];
    let messages_hash = calculate_messages_sent_hash(&messages_sent);
    let expected_hash =
        StarkFelt::try_from("0x00c89474a9007dc060aed76caf8b30b927cfea1ebce2d134b943b8d7121004e4")
            .unwrap();
    assert_eq!(messages_hash, expected_hash);
}

#[test]
fn test_revert_reason_hash_regression() {
    let execution_succeeded = TransactionExecutionStatus::Succeeded;
    assert_eq!(get_revert_reason_hash(&execution_succeeded), StarkFelt::ZERO);
    let execution_reverted =
        TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
            revert_reason: "ABC".to_string(),
        });
    let expected_hash =
        StarkFelt::try_from("0x01629b9dda060bb30c7908346f6af189c16773fa148d3366701fbaa35d54f3c8")
            .unwrap();
    assert_eq!(get_revert_reason_hash(&execution_reverted), expected_hash);
}
