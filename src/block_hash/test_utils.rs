use std::collections::HashMap;

use indexmap::indexmap;
use primitive_types::H160;

use super::transaction_commitment::TransactionLeafElement;
use crate::block::{BlockHash, BlockNumber};
use crate::block_hash::event_commitment::EventLeafElement;
use crate::core::{ClassHash, CompiledClassHash, ContractAddress, EthAddress, Nonce};
use crate::hash::StarkFelt;
use crate::state::ThinStateDiff;
use crate::transaction::{
    Builtin, Event, EventContent, EventData, EventKey, ExecutionResources, Fee,
    InvokeTransactionOutput, L2ToL1Payload, MessageToL1, RevertedTransactionExecutionStatus,
    TransactionExecutionStatus, TransactionHash, TransactionOutput, TransactionReceipt,
    TransactionSignature,
};

pub(crate) fn get_transaction_leaf_element() -> TransactionLeafElement {
    let transaction_hash = TransactionHash(StarkFelt::ONE);
    let transaction_signature = TransactionSignature(vec![StarkFelt::TWO, StarkFelt::THREE]);
    TransactionLeafElement { transaction_hash, transaction_signature }
}

pub(crate) fn get_event_leaf_element(seed: u8) -> EventLeafElement {
    EventLeafElement {
        event: Event {
            from_address: ContractAddress::from(seed + 8),
            content: EventContent {
                keys: [seed, seed + 1].iter().map(|key| EventKey(StarkFelt::from(*key))).collect(),
                data: EventData(
                    [seed + 2, seed + 3, seed + 4].into_iter().map(StarkFelt::from).collect(),
                ),
            },
        },
        transaction_hash: TransactionHash(StarkFelt::from(4660_u16)),
    }
}

pub(crate) fn get_transaction_reciept() -> TransactionReceipt {
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
    TransactionReceipt {
        transaction_hash: TransactionHash(StarkFelt::from(1234_u16)),
        block_hash: BlockHash(StarkFelt::from(5678_u16)),
        block_number: BlockNumber(99),
        output: invoke_output,
    }
}

pub(crate) fn generate_message_to_l1(seed: u64) -> MessageToL1 {
    MessageToL1 {
        from_address: ContractAddress::from(seed),
        to_address: EthAddress(H160::from_low_u64_be(seed + 1)),
        payload: L2ToL1Payload(vec![StarkFelt::from(seed + 2), StarkFelt::from(seed + 3)]),
    }
}

pub(crate) fn get_state_diff() -> ThinStateDiff {
    ThinStateDiff {
        deployed_contracts: indexmap! {
            0u64.into() => ClassHash(1u64.into()),
            2u64.into() => ClassHash(3u64.into()),
        },
        storage_diffs: indexmap! {
            4u64.into() => indexmap! {
                5u64.into() => 6u64.into(),
                7u64.into() => 8u64.into(),
            },
            9u64.into() => indexmap! {
                10u64.into() => 11u64.into(),
            },
        },
        declared_classes: indexmap! {
            ClassHash(12u64.into()) => CompiledClassHash(13u64.into()),
            ClassHash(14u64.into()) => CompiledClassHash(15u64.into()),
        },
        deprecated_declared_classes: vec![ClassHash(16u64.into())],
        nonces: indexmap! {
            17u64.into() => Nonce(18u64.into()),
        },
        replaced_classes: indexmap! {
            19u64.into() => ClassHash(20u64.into()),
        },
    }
}
