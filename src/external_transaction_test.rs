use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce};
use crate::external_transaction::{
    ContractClass, DataAvailabilityMode, DeclareType, DeployAccountType,
    ExternalDeclareTransaction, ExternalDeclareTransactionV3, ExternalDeployAccountTransaction,
    ExternalDeployAccountTransactionV3, ExternalInvokeTransaction, ExternalInvokeTransactionV3,
    ExternalTransaction, InvokeType,
};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, ResourceBoundsMapping,
    Tip, TransactionSignature, TransactionVersion,
};

fn create_default_declare_v3() -> ExternalDeclareTransaction {
    ExternalDeclareTransaction::V3(ExternalDeclareTransactionV3 {
        contract_class: ContractClass::default(),
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip::default(),
        signature: TransactionSignature::default(),
        nonce: Nonce::default(),
        compiled_class_hash: CompiledClassHash::default(),
        sender_address: ContractAddress::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData::default(),
        account_deployment_data: AccountDeploymentData::default(),
        version: TransactionVersion::THREE,
        r#type: DeclareType::default(),
    })
}

fn create_default_deploy_account_v3() -> ExternalDeployAccountTransaction {
    ExternalDeployAccountTransaction::V3(ExternalDeployAccountTransactionV3 {
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip::default(),
        contract_address_salt: ContractAddressSalt::default(),
        class_hash: ClassHash::default(),
        constructor_calldata: Calldata::default(),
        nonce: Nonce::default(),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData::default(),
        version: TransactionVersion::THREE,
        r#type: DeployAccountType::default(),
    })
}

fn create_default_invoke_v3() -> ExternalInvokeTransaction {
    ExternalInvokeTransaction::V3(ExternalInvokeTransactionV3 {
        resource_bounds: ResourceBoundsMapping::default(),
        tip: Tip::default(),
        calldata: Calldata::default(),
        sender_address: ContractAddress::default(),
        nonce: Nonce::default(),
        signature: TransactionSignature::default(),
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        paymaster_data: PaymasterData::default(),
        account_deployment_data: AccountDeploymentData::default(),
        version: TransactionVersion::THREE,
        r#type: InvokeType::default(),
    })
}

#[test]
fn test_transactions() {
    let test_cases: Vec<(&str, ExternalTransaction)> = vec![
        ("DeclareTransactionV3", ExternalTransaction::Declare(create_default_declare_v3())),
        (
            "DeployAccountTransactionV3",
            ExternalTransaction::DeployAccount(create_default_deploy_account_v3()),
        ),
        ("InvokeTransactionV3", ExternalTransaction::Invoke(create_default_invoke_v3())),
    ];

    for (name, expected_tx) in test_cases {
        let serialized = serde_json::to_string(&expected_tx).unwrap();
        let deserialized: ExternalTransaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(expected_tx, deserialized, "Failed test case: {}", name);
    }
}
