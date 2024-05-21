use std::collections::HashMap;

use primitive_types::H160;

use crate::core::{ContractAddress, EthAddress};
use crate::hash::StarkFelt;
use crate::transaction::{
    Builtin, ExecutionResources, Fee, L2ToL1Payload, MessageToL1,
    RevertedTransactionExecutionStatus, TransactionExecutionStatus, TransactionOutputCommon,
};

pub(crate) fn get_transaction_output() -> TransactionOutputCommon {
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
    TransactionOutputCommon {
        actual_fee: Fee(99804),
        messages_sent: vec![generate_message_to_l1(34), generate_message_to_l1(56)],
        events: vec![],
        execution_status,
        execution_resources,
    }
}

pub(crate) fn generate_message_to_l1(seed: u64) -> MessageToL1 {
    MessageToL1 {
        from_address: ContractAddress::from(seed),
        to_address: EthAddress(H160::from_low_u64_be(seed + 1)),
        payload: L2ToL1Payload(vec![StarkFelt::from(seed + 2), StarkFelt::from(seed + 3)]),
    }
}
