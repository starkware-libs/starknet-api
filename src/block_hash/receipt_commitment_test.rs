use std::collections::HashMap;

use primitive_types::H160;
use starknet_types_core::felt::Felt;
use starknet_types_core::hash::Poseidon;

use super::calculate_messages_sent_hash;
use crate::block::{BlockHash, BlockNumber};
use crate::block_hash::receipt_commitment::{
    calculate_receipt_commitment, calculate_receipt_hash, get_revert_reason,
};
use crate::core::{ContractAddress, EthAddress, ReceiptCommitment};
use crate::transaction::{
    Builtin, ExecutionResources, Fee, InvokeTransactionOutput, L2ToL1Payload, MessageToL1,
    RevertedTransactionExecutionStatus, TransactionExecutionStatus, TransactionHash,
    TransactionOutput, TransactionReceipt,
};

#[test]
fn test_receipt_hash_regression() {
    let execution_status =
        TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
            revert_reason: "aborted".to_string(),
        });
    let execution_resources = ExecutionResources {
        steps: 98,
        builtin_instance_counter: HashMap::from([(Builtin::Bitwise, 11), (Builtin::EcOp, 22)]),
        memory_holes: 76,
        da_l1_gas_consumed: 54,
        da_l1_data_gas_consumed: 32,
    };
    let invoke_output = TransactionOutput::Invoke(InvokeTransactionOutput {
        actual_fee: Fee(12),
        messages_sent: vec![generate_message_to_l1(34), generate_message_to_l1(56)],
        events: vec![],
        execution_status,
        execution_resources,
    });
    let transaction_receipt = TransactionReceipt {
        transaction_hash: TransactionHash(Felt::from(1234_u16)),
        block_hash: BlockHash(Felt::from(5678_u16)),
        block_number: BlockNumber(99),
        output: invoke_output,
    };

    let expected_hash = Felt::from_hex_unchecked(
        "0x0144fc581761cdaa8327d6d32d4120cddda36be0fda68e463efe0d49f141ccc7",
    );
    assert_eq!(calculate_receipt_hash(&transaction_receipt), expected_hash);

    let expected_root = ReceiptCommitment(Felt::from_hex_unchecked(
        "0x03db8b2c479130ff1f1d718c593ee164849f89686e1a4c82e8a6abbf506852e8",
    ));
    assert_eq!(calculate_receipt_commitment::<Poseidon>(&[transaction_receipt]), expected_root);
}

#[test]
fn test_messages_sent_regression() {
    let messages_sent = vec![generate_message_to_l1(0), generate_message_to_l1(1)];
    let messages_hash = calculate_messages_sent_hash(&messages_sent);
    let expected_hash = Felt::from_hex_unchecked(
        "0x00c89474a9007dc060aed76caf8b30b927cfea1ebce2d134b943b8d7121004e4",
    );
    assert_eq!(messages_hash, expected_hash);
}

fn generate_message_to_l1(seed: u64) -> MessageToL1 {
    MessageToL1 {
        from_address: ContractAddress::from(seed),
        to_address: EthAddress(H160::from_low_u64_be(seed + 1)),
        payload: L2ToL1Payload(vec![Felt::from(seed + 2), Felt::from(seed + 3)]),
    }
}

#[test]
fn test_revert_reason_regression() {
    let execution_succeeded = TransactionExecutionStatus::Succeeded;
    assert_eq!(get_revert_reason(&execution_succeeded), Felt::ZERO);
    let execution_reverted =
        TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
            revert_reason: "ABC".to_string(),
        });
    assert_eq!(get_revert_reason(&execution_reverted), Felt::from(0x414243_u32));
}
