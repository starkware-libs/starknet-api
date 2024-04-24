use std::sync::Arc;

use rstest::rstest;

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce, PatriciaKey};
use crate::hash::{StarkFelt, StarkHash};
use crate::rpc_transaction::{
    ContractClass, DataAvailabilityMode, RPCDeclareTransaction, RPCDeclareTransactionV3,
    RPCDeployAccountTransaction, RPCDeployAccountTransactionV3, RPCInvokeTransaction,
    RPCInvokeTransactionV3, RPCTransaction, ResourceBoundsMapping,
};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, Tip, TransactionSignature,
};
use crate::{contract_address, patricia_key, stark_felt};

fn create_declare_v3() -> RPCDeclareTransaction {
    RPCDeclareTransaction::V3(RPCDeclareTransactionV3 {
        contract_class: ContractClass::default(),
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip(1),
        signature: TransactionSignature(vec![StarkFelt::ONE, StarkFelt::TWO]),
        nonce: Nonce(stark_felt!("0x1")),
        compiled_class_hash: CompiledClassHash(stark_felt!("0x2")),
        sender_address: contract_address!("0x3"),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L2,
        paymaster_data: PaymasterData(vec![StarkFelt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![StarkFelt::THREE]),
    })
}

fn create_deploy_account_v3() -> RPCDeployAccountTransaction {
    RPCDeployAccountTransaction::V3(RPCDeployAccountTransactionV3 {
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip::default(),
        contract_address_salt: ContractAddressSalt(stark_felt!("0x23")),
        class_hash: ClassHash(stark_felt!("0x2")),
        constructor_calldata: Calldata(Arc::new(vec![StarkFelt::ZERO])),
        nonce: Nonce(stark_felt!("0x60")),
        signature: TransactionSignature(vec![StarkFelt::TWO]),
        nonce_data_availability_mode: DataAvailabilityMode::L2,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![StarkFelt::TWO, StarkFelt::ZERO]),
    })
}

fn create_invoke_v3() -> RPCInvokeTransaction {
    RPCInvokeTransaction::V3(RPCInvokeTransactionV3 {
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip(50),
        calldata: Calldata(Arc::new(vec![stark_felt!("0x2000"), stark_felt!("0x1000")])),
        sender_address: contract_address!("0x53"),
        nonce: Nonce(stark_felt!("0x32")),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![StarkFelt::TWO, StarkFelt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![stark_felt!("0x87")]),
    })
}

#[rstest]
#[case(RPCTransaction::Declare(create_declare_v3()))]
#[case(RPCTransaction::DeployAccount(create_deploy_account_v3()))]
#[case(RPCTransaction::Invoke(create_invoke_v3()))]
fn test_rpc_transactions(#[case] tx: RPCTransaction) {
    let serialized = serde_json::to_string(&tx).unwrap();
    let deserialized: RPCTransaction = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tx, deserialized);
}
