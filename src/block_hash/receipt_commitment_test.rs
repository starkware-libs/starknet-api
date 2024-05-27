use starknet_types_core::felt::Felt;
use starknet_types_core::hash::Poseidon;

use super::calculate_messages_sent_hash;
use crate::block::{GasPrice, GasPricePerToken};
use crate::block_hash::receipt_commitment::{
    calculate_receipt_commitment, calculate_receipt_hash, get_revert_reason_hash, ReceiptElement,
};
use crate::core::ReceiptCommitment;
use crate::felt;
use crate::hash::{FeltConverter, TryIntoFelt};
use crate::block_hash::test_utils::{generate_message_to_l1, get_transaction_output};
use crate::transaction::{
    RevertedTransactionExecutionStatus, TransactionExecutionStatus, TransactionHash,
    TransactionVersion,
};

#[test]
fn test_receipt_hash_regression() {
    let mut transaction_receipt = ReceiptElement {
        transaction_hash: TransactionHash(Felt::from(1234_u16)),
        transaction_output: get_transaction_output(),
        transaction_version: TransactionVersion::TWO,
    };
    let l1_data_gas_price =
        GasPricePerToken { price_in_fri: GasPrice(123), price_in_wei: GasPrice(456) };
    let l1_gas_price =
        GasPricePerToken { price_in_fri: GasPrice(456), price_in_wei: GasPrice(789) };

    let expected_hash = felt!("0x06cb27bfc55dee54e6d0fc7a6790e39f0f3c003576d50f7b8e8a1be24c351bcf");
    assert_eq!(
        calculate_receipt_hash(&transaction_receipt, l1_data_gas_price, l1_gas_price),
        expected_hash
    );

    let expected_root = ReceiptCommitment(
        felt!("0x03a0af1272fc3b0b83894fd7b6b70d89acb07772bc28efc9091e3cc1c2c72493")
    );

    // Test for a V3 transactions.
    transaction_receipt.transaction_version = TransactionVersion::THREE;
    assert_eq!(
        calculate_receipt_commitment::<Poseidon>(
            &[transaction_receipt],
            l1_data_gas_price,
            l1_gas_price
        ),
        expected_root
    );
}

#[test]
fn test_messages_sent_regression() {
    let messages_sent = vec![generate_message_to_l1(0), generate_message_to_l1(1)];
    let messages_hash = calculate_messages_sent_hash(&messages_sent);
    let expected_hash = felt!("0x00c89474a9007dc060aed76caf8b30b927cfea1ebce2d134b943b8d7121004e4");
    assert_eq!(messages_hash, expected_hash);
}

#[test]
fn test_revert_reason_hash_regression() {
    let execution_succeeded = TransactionExecutionStatus::Succeeded;
    assert_eq!(get_revert_reason_hash(&execution_succeeded), Felt::ZERO);
    let execution_reverted =
        TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
            revert_reason: "ABC".to_string(),
        });
    let expected_hash = felt!("0x01629b9dda060bb30c7908346f6af189c16773fa148d3366701fbaa35d54f3c8");
    assert_eq!(get_revert_reason_hash(&execution_reverted), expected_hash);
}
