use crate::core::{ClassHash, ContractAddress};
use crate::external_transaction::{
    ExternalDeclareTransaction, ExternalDeclareTransactionV3, ExternalDeployAccountTransaction,
    ExternalDeployAccountTransactionV3, ExternalInvokeTransaction, ExternalInvokeTransactionV3,
    ExternalTransaction,
};
use crate::state::ContractClass;
use crate::transaction::TransactionHash;

/// Represents a paid Starknet transaction.
#[derive(Debug)]
pub enum InternalTransaction {
    Declare(InternalDeclareTransaction),
    DeployAccount(InternalDeployAccountTransaction),
    Invoke(InternalInvokeTransaction),
}

#[derive(Debug)]
pub struct InternalDeclareTransaction {
    pub tx: crate::transaction::DeclareTransaction,
    pub tx_hash: TransactionHash,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
    pub class_info: ClassInfo,
}

#[derive(Debug, Clone)]
pub struct InternalDeployAccountTransaction {
    pub tx: crate::transaction::DeployAccountTransaction,
    pub tx_hash: TransactionHash,
    pub contract_address: ContractAddress,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
}

#[derive(Debug, Clone)]
pub struct InternalInvokeTransaction {
    pub tx: crate::transaction::InvokeTransaction,
    pub tx_hash: TransactionHash,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub contract_class: ContractClass,
    pub sierra_program_length: usize,
    pub abi_length: usize,
}

impl From<ExternalTransaction> for InternalTransaction {
    fn from(value: ExternalTransaction) -> Self {
        match value {
            ExternalTransaction::Declare(tx) => InternalTransaction::Declare(tx.into()),
            ExternalTransaction::DeployAccount(tx) => InternalTransaction::DeployAccount(tx.into()),
            ExternalTransaction::Invoke(tx) => InternalTransaction::Invoke(tx.into()),
        }
    }
}

impl From<ExternalDeclareTransaction> for InternalDeclareTransaction {
    fn from(value: ExternalDeclareTransaction) -> Self {
        match value {
            ExternalDeclareTransaction::V3(tx) => InternalDeclareTransaction::from_v3(tx),
        }
    }
}

impl From<ExternalDeployAccountTransaction> for InternalDeployAccountTransaction {
    fn from(value: ExternalDeployAccountTransaction) -> Self {
        match value {
            ExternalDeployAccountTransaction::V3(tx_v3) => {
                InternalDeployAccountTransaction::from_v3(tx_v3)
            }
        }
    }
}

impl From<ExternalInvokeTransaction> for InternalInvokeTransaction {
    fn from(value: ExternalInvokeTransaction) -> Self {
        match value {
            ExternalInvokeTransaction::V3(tx) => InternalInvokeTransaction::from_v3(tx),
        }
    }
}

impl InternalDeclareTransaction {
    fn from_v3(external_tx: ExternalDeclareTransactionV3) -> Self {
        let class_hash = ClassHash::default(); // TODO.

        let tx =
            crate::transaction::DeclareTransaction::V3(crate::transaction::DeclareTransactionV3 {
                resource_bounds: external_tx.resource_bounds,
                tip: external_tx.tip,
                signature: external_tx.signature,
                nonce: external_tx.nonce,
                class_hash,
                compiled_class_hash: external_tx.compiled_class_hash,
                sender_address: external_tx.sender_address,
                nonce_data_availability_mode: external_tx.nonce_data_availability_mode,
                fee_data_availability_mode: external_tx.fee_data_availability_mode,
                paymaster_data: external_tx.paymaster_data,
                account_deployment_data: external_tx.account_deployment_data,
            });

        let tx_hash = TransactionHash::default(); // TODO: Add Transaction hash calculation.

        // TODO: convert contract class to Internal type.
        let contract_class = ContractClass {
            sierra_program: vec![],
            abi: external_tx.contract_class.abi,
            entry_points_by_type: external_tx.contract_class.entry_points_by_type,
        };

        InternalDeclareTransaction {
            tx,
            tx_hash,
            only_query: false,
            class_info: ClassInfo { abi_length: 0, contract_class, sierra_program_length: 0 },
        }
    }
}

impl InternalDeployAccountTransaction {
    fn from_v3(external_tx: ExternalDeployAccountTransactionV3) -> Self {
        let tx = crate::transaction::DeployAccountTransaction::V3(
            crate::transaction::DeployAccountTransactionV3 {
                resource_bounds: external_tx.resource_bounds,
                tip: external_tx.tip,
                paymaster_data: external_tx.paymaster_data,
                nonce_data_availability_mode: external_tx.nonce_data_availability_mode,
                fee_data_availability_mode: external_tx.fee_data_availability_mode,
                signature: external_tx.signature,
                nonce: external_tx.nonce,
                class_hash: external_tx.class_hash,
                constructor_calldata: external_tx.constructor_calldata,
                contract_address_salt: external_tx.contract_address_salt,
            },
        );

        let tx_hash = TransactionHash::default(); // TODO.
        let contract_address = ContractAddress::default(); // TODO.

        InternalDeployAccountTransaction { tx, tx_hash, contract_address, only_query: false }
    }
}

impl InternalInvokeTransaction {
    fn from_v3(external_tx: ExternalInvokeTransactionV3) -> Self {
        let tx =
            crate::transaction::InvokeTransaction::V3(crate::transaction::InvokeTransactionV3 {
                resource_bounds: external_tx.resource_bounds,
                tip: external_tx.tip,
                signature: external_tx.signature,
                nonce: external_tx.nonce,
                sender_address: external_tx.sender_address,
                calldata: external_tx.calldata,
                nonce_data_availability_mode: external_tx.nonce_data_availability_mode,
                fee_data_availability_mode: external_tx.fee_data_availability_mode,
                paymaster_data: external_tx.paymaster_data,
                account_deployment_data: external_tx.account_deployment_data,
            });
        let tx_hash = TransactionHash::default(); // TODO: Add Transaction hash calculation.

        InternalInvokeTransaction { tx, tx_hash, only_query: false }
    }
}
