#[cfg(test)]
#[path = "external_transaction_test.rs"]
mod external_transaction_test;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::{
    calculate_contract_address, ClassHash, CompiledClassHash, ContractAddress, Nonce,
};
use crate::data_availability::DataAvailabilityMode;
use crate::internal_transaction::{
    ClassInfo, InternalDeclareTransaction, InternalDeployAccountTransaction,
    InternalInvokeTransaction, InternalTransaction,
};
use crate::state::{ContractClass as InternalContractClass, EntryPoint, EntryPointType};
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, ResourceBoundsMapping,
    Tip, TransactionHash, TransactionSignature,
};

/// An external transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ExternalTransaction {
    /// A declare transaction.
    #[serde(rename = "DECLARE")]
    Declare(ExternalDeclareTransaction),
    /// A deploy account transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(ExternalDeployAccountTransaction),
    /// An invoke transaction.
    #[serde(rename = "INVOKE_FUNCTION")]
    Invoke(ExternalInvokeTransaction),
}

/// A declare transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "version")]
pub enum ExternalDeclareTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalDeclareTransactionV3),
}

/// A deploy account transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "version")]
pub enum ExternalDeployAccountTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalDeployAccountTransactionV3),
}

/// An invoke transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "version")]
pub enum ExternalInvokeTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalInvokeTransactionV3),
}

/// A declare transaction of a Cairo-v1 contract class that can be added to Starknet through the
/// Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeclareTransactionV3 {
    pub contract_class: ContractClass,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub compiled_class_hash: CompiledClassHash,
    pub sender_address: ContractAddress,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
}

/// A deploy account transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeployAccountTransactionV3 {
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub contract_address_salt: ContractAddressSalt,
    pub class_hash: ClassHash,
    pub constructor_calldata: Calldata,
    pub nonce: Nonce,
    pub signature: TransactionSignature,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
}

/// An invoke account transaction that can be added to Starknet through the Starknet gateway.
/// The invoke is a V3 transaction.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalInvokeTransactionV3 {
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub calldata: Calldata,
    pub sender_address: ContractAddress,
    pub nonce: Nonce,
    pub signature: TransactionSignature,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractClass {
    #[serde(rename = "sierra_program")]
    pub compressed_sierra_program: String,
    pub contract_class_version: String,
    pub entry_points_by_type: HashMap<EntryPointType, Vec<EntryPoint>>,
    pub abi: String,
}

impl ExternalTransaction {
    pub fn into_internal(self) -> InternalTransaction {
        match self {
            ExternalTransaction::Declare(tx) => InternalTransaction::Declare(tx.into_internal()),
            ExternalTransaction::DeployAccount(tx) => {
                InternalTransaction::DeployAccount(tx.into_internal())
            }
            ExternalTransaction::Invoke(tx) => InternalTransaction::Invoke(tx.into_internal()),
        }
    }
}

impl ExternalDeclareTransaction {
    pub fn into_internal(self) -> InternalDeclareTransaction {
        match self {
            ExternalDeclareTransaction::V3(tx) => tx.into_internal(),
        }
    }
}

impl ExternalDeployAccountTransaction {
    pub fn into_internal(self) -> InternalDeployAccountTransaction {
        match self {
            ExternalDeployAccountTransaction::V3(tx) => tx.into_internal(),
        }
    }
}

impl ExternalInvokeTransaction {
    fn into_internal(self) -> InternalInvokeTransaction {
        match self {
            ExternalInvokeTransaction::V3(tx) => tx.into_internal(),
        }
    }
}

impl ExternalDeclareTransactionV3 {
    fn into_internal(self) -> InternalDeclareTransaction {
        let class_hash = calculate_class_hash();

        let tx =
            crate::transaction::DeclareTransaction::V3(crate::transaction::DeclareTransactionV3 {
                resource_bounds: self.resource_bounds,
                tip: self.tip,
                signature: self.signature,
                nonce: self.nonce,
                class_hash,
                compiled_class_hash: self.compiled_class_hash,
                sender_address: self.sender_address,
                nonce_data_availability_mode: self.nonce_data_availability_mode,
                fee_data_availability_mode: self.fee_data_availability_mode,
                paymaster_data: self.paymaster_data,
                account_deployment_data: self.account_deployment_data,
            });

        let tx_hash = calculate_transction_hash();

        InternalDeclareTransaction {
            tx,
            tx_hash,
            only_query: false,
            // TODO: convert contract class to Internal type.
            // TODO: calculate abi and sierra program lengths.
            class_info: ClassInfo {
                abi_length: 0,
                contract_class: InternalContractClass::default(),
                sierra_program_length: 0,
            },
        }
    }
}

impl ExternalDeployAccountTransactionV3 {
    fn into_internal(self) -> InternalDeployAccountTransaction {
        let tx = crate::transaction::DeployAccountTransaction::V3(
            crate::transaction::DeployAccountTransactionV3 {
                resource_bounds: self.resource_bounds,
                tip: self.tip,
                paymaster_data: self.paymaster_data,
                nonce_data_availability_mode: self.nonce_data_availability_mode,
                fee_data_availability_mode: self.fee_data_availability_mode,
                signature: self.signature,
                nonce: self.nonce,
                class_hash: self.class_hash,
                constructor_calldata: self.constructor_calldata.clone(),
                contract_address_salt: self.contract_address_salt,
            },
        );

        let tx_hash = calculate_transction_hash();
        let contract_address = calculate_contract_address(
            self.contract_address_salt,
            self.class_hash,
            &self.constructor_calldata,
            ContractAddress::default(),
        )
        .unwrap();

        InternalDeployAccountTransaction { tx, tx_hash, contract_address, only_query: false }
    }
}

impl ExternalInvokeTransactionV3 {
    fn into_internal(self) -> InternalInvokeTransaction {
        let tx =
            crate::transaction::InvokeTransaction::V3(crate::transaction::InvokeTransactionV3 {
                resource_bounds: self.resource_bounds,
                tip: self.tip,
                signature: self.signature,
                nonce: self.nonce,
                sender_address: self.sender_address,
                calldata: self.calldata,
                nonce_data_availability_mode: self.nonce_data_availability_mode,
                fee_data_availability_mode: self.fee_data_availability_mode,
                paymaster_data: self.paymaster_data,
                account_deployment_data: self.account_deployment_data,
            });
        let tx_hash = calculate_transction_hash();

        InternalInvokeTransaction { tx, tx_hash, only_query: false }
    }
}

fn calculate_class_hash() -> ClassHash {
    // TODO: add class hash calculation.
    ClassHash::default()
}

fn calculate_transction_hash() -> TransactionHash {
    // TODO: add transaction hash calculation.
    TransactionHash::default()
}
