use std::collections::BTreeMap;
use std::sync::Arc;

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce, PatriciaKey};
use crate::external_transaction::{
    ContractClass, DataAvailabilityMode, DeclareType, DeployAccountType,
    ExternalDeclareTransaction, ExternalDeclareTransactionV3, ExternalDeployAccountTransaction,
    ExternalDeployAccountTransactionV3, ExternalInvokeTransaction, ExternalInvokeTransactionV3,
    ExternalTransaction, InvokeType,
};
use crate::hash::{StarkFelt, StarkHash};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, Resource, ResourceBounds,
    ResourceBoundsMapping, Tip, TransactionSignature, TransactionVersion,
};
use crate::{contract_address, patricia_key, stark_felt};

fn create_resource_bounds() -> ResourceBoundsMapping {
    let mut map = BTreeMap::new();
    map.insert(Resource::L1Gas, ResourceBounds { max_amount: 100, max_price_per_unit: 12 });
    map.insert(Resource::L2Gas, ResourceBounds { max_amount: 58, max_price_per_unit: 31 });
    ResourceBoundsMapping(map)
}

fn create_declare_v3() -> ExternalDeclareTransaction {
    ExternalDeclareTransaction::V3(ExternalDeclareTransactionV3 {
        contract_class: ContractClass::default(),
        resource_bounds: create_resource_bounds(),
        tip: Tip(1),
        signature: TransactionSignature(vec![StarkFelt::ONE, StarkFelt::TWO]),
        nonce: Nonce(stark_felt!("0x1")),
        compiled_class_hash: CompiledClassHash(stark_felt!("0x2")),
        sender_address: contract_address!("0x3"),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L2,
        paymaster_data: PaymasterData(vec![StarkFelt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![StarkFelt::THREE]),
        version: TransactionVersion::THREE,
        r#type: DeclareType::default(),
    })
}

fn create_deploy_account_v3() -> ExternalDeployAccountTransaction {
    ExternalDeployAccountTransaction::V3(ExternalDeployAccountTransactionV3 {
        resource_bounds: create_resource_bounds(),
        tip: Tip::default(),
        contract_address_salt: ContractAddressSalt(stark_felt!("0x23")),
        class_hash: ClassHash(stark_felt!("0x2")),
        constructor_calldata: Calldata(Arc::new(vec![StarkFelt::ZERO])),
        nonce: Nonce(stark_felt!("0x60")),
        signature: TransactionSignature(vec![StarkFelt::TWO]),
        nonce_data_availability_mode: DataAvailabilityMode::L2,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![StarkFelt::TWO, StarkFelt::ZERO]),
        version: TransactionVersion::THREE,
        r#type: DeployAccountType::default(),
    })
}

fn create_invoke_v3() -> ExternalInvokeTransaction {
    ExternalInvokeTransaction::V3(ExternalInvokeTransactionV3 {
        resource_bounds: create_resource_bounds(),
        tip: Tip(50),
        calldata: Calldata(Arc::new(vec![stark_felt!("0x2000"), stark_felt!("0x1000")])),
        sender_address: contract_address!("0x53"),
        nonce: Nonce(stark_felt!("0x32")),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData(vec![StarkFelt::TWO, StarkFelt::ZERO]),
        account_deployment_data: AccountDeploymentData(vec![stark_felt!("0x87")]),
        version: TransactionVersion::THREE,
        r#type: InvokeType::default(),
    })
}

#[test]
fn test_external_transactions() {
    let test_cases: Vec<(&str, ExternalTransaction)> = vec![
        ("DeclareTransactionV3", ExternalTransaction::Declare(create_declare_v3())),
        (
            "DeployAccountTransactionV3",
            ExternalTransaction::DeployAccount(create_deploy_account_v3()),
        ),
        ("InvokeTransactionV3", ExternalTransaction::Invoke(create_invoke_v3())),
    ];

    for (name, expected_tx) in test_cases {
        let serialized = serde_json::to_string(&expected_tx).unwrap();
        let deserialized: ExternalTransaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(expected_tx, deserialized, "Failed test case: {}", name);
    }
}
