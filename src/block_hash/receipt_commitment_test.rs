use std::collections::HashMap;

use primitive_types::H160;

use super::calculate_messages_sent_hash;
use crate::block::{BlockHash, BlockNumber};
use crate::block_hash::receipt_commitment::{
    as_stark_felt, calculate_receipt_commitment, calculate_receipt_hash, get_revert_reason,
};
use crate::core::{ContractAddress, EthAddress, ReceiptCommitment};
use crate::hash::{PoseidonHashCalculator, StarkFelt};
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
        transaction_hash: TransactionHash(StarkFelt::from(1234_u16)),
        block_hash: BlockHash(StarkFelt::from(5678_u16)),
        block_number: BlockNumber(99),
        output: invoke_output,
    };

    let expected_hash =
        StarkFelt::try_from("0x06e6572f34d47e2f12acb27e7ee6a6f17117de19ba43cafa4d21a46b925afeeb")
            .unwrap();
    assert_eq!(calculate_receipt_hash(&transaction_receipt), expected_hash);

    let expected_root = ReceiptCommitment(
        StarkFelt::try_from("0x07c0ed4255e30b39c90dab4fb821c74bc08784a4dfda37a679a7c4b401990103")
            .unwrap(),
    );
    assert_eq!(
        calculate_receipt_commitment::<PoseidonHashCalculator>(&[transaction_receipt]),
        expected_root
    );
}

#[test]
fn test_extend_to_stark_felt() {
    let mut one_as_bytes = [0_u8; 20];
    one_as_bytes[19] = 1;
    let one_as_felt = as_stark_felt(&one_as_bytes);
    assert_eq!(one_as_felt, StarkFelt::ONE);
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

fn generate_message_to_l1(seed: u64) -> MessageToL1 {
    MessageToL1 {
        from_address: ContractAddress::from(seed),
        to_address: EthAddress(H160::from_low_u64_be(seed + 1)),
        payload: L2ToL1Payload(vec![StarkFelt::from(seed + 2), StarkFelt::from(seed + 3)]),
    }
}

#[test]
fn test_revert_reason_regression() {
    let execution_succeeded = TransactionExecutionStatus::Succeeded;
    assert_eq!(get_revert_reason(&execution_succeeded), StarkFelt::ZERO);
    let execution_reverted =
        TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
            revert_reason: "ABC".to_string(),
        });
    assert_eq!(get_revert_reason(&execution_reverted), StarkFelt::from(0x414243_u32));
}
