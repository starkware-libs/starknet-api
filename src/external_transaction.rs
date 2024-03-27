use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::core::{ClassHash, CompiledClassHash, ContractAddress, EntryPointSelector, Nonce};
use crate::deprecated_contract_class::{
    ContractClassAbiEntry as DeprecatedContractClassAbiEntry, EntryPoint as DeprecatedEntryPoint,
    EntryPointType as DeprecatedEntryPointType,
};
use crate::state::{EntryPoint, EntryPointType};
// use crate::internal_transaction::ContractClass;
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, Fee, PaymasterData,
    ResourceBoundsMapping, Tip, TransactionSignature, TransactionVersion,
};
/// An external transaction.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExternalTransaction {
    /// A declare transaction.
    Declare(ExternalDeclareTransaction),
    /// A deploy account transaction.
    DeployAccount(ExternalDeployAccountTransaction),
    /// An invoke transaction.
    Invoke(ExternalInvokeTransaction),
}

// Each transaction type has a field called `type`. This field needs to be of a type that
// serializes to/deserializes from a constant string.
//
// The reason we don't solve this by having an enum of a generic transaction and let serde generate
// the `type` field through #[serde(tag)] is because we want to serialize/deserialize from the
// structs of the specific transaction types.

/// The type field of a deploy account transaction. This enum serializes/deserializes into a
/// constant string.
#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
pub enum DeployAccountType {
    #[serde(rename = "DEPLOY_ACCOUNT")]
    #[default]
    DeployAccount,
}

/// The type field of an invoke transaction. This enum serializes/deserializes into a constant
/// string.
#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
pub enum InvokeType {
    #[serde(rename = "INVOKE_FUNCTION")]
    #[default]
    Invoke,
}

/// The type field of a declare transaction. This enum serializes/deserializes into a constant
/// string.
#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
pub enum DeclareType {
    #[serde(rename = "DECLARE")]
    #[default]
    Declare,
}

/// A deploy account transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeployAccountTransactionV1 {
    pub contract_address_salt: ContractAddressSalt,
    pub class_hash: ClassHash,
    pub constructor_calldata: Calldata,
    pub nonce: Nonce,
    pub max_fee: Fee,
    pub signature: TransactionSignature,
    pub version: TransactionVersion,
    pub r#type: DeployAccountType,
}

/// A deploy account transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeployAccountTransactionV3 {
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub contract_address_salt: ContractAddressSalt,
    pub class_hash: ClassHash,
    pub constructor_calldata: Calldata,
    pub nonce: Nonce,
    pub signature: TransactionSignature,
    pub nonce_data_availability_mode: ReservedDataAvailabilityMode,
    pub fee_data_availability_mode: ReservedDataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub version: TransactionVersion,
    pub r#type: DeployAccountType,
}

/// A deploy account transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum ExternalDeployAccountTransaction {
    V1(ExternalDeployAccountTransactionV1),
    V3(ExternalDeployAccountTransactionV3),
}

/// An invoke account transaction that can be added to Starknet through the Starknet gateway.
/// The invoke is a V0 transaction.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
// TODO(Shahak): Add tests for invoke v0.
#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalInvokeTransactionV0 {
    pub calldata: Calldata,
    pub contract_address: ContractAddress,
    pub max_fee: Fee,
    pub signature: TransactionSignature,
    pub version: TransactionVersion,
    pub r#type: InvokeType,
    pub entry_point_selector: EntryPointSelector,
}

/// An invoke account transaction that can be added to Starknet through the Starknet gateway.
/// The invoke is a V1 transaction.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalInvokeTransactionV1 {
    pub calldata: Calldata,
    pub sender_address: ContractAddress,
    pub nonce: Nonce,
    pub max_fee: Fee,
    pub signature: TransactionSignature,
    pub version: TransactionVersion,
    pub r#type: InvokeType,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ReservedDataAvailabilityMode {
    Reserved = 0,
}

/// An invoke account transaction that can be added to Starknet through the Starknet gateway.
/// The invoke is a V3 transaction.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
// TODO(Shahak): Add tests for invoke v3.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalInvokeTransactionV3 {
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub calldata: Calldata,
    pub sender_address: ContractAddress,
    pub nonce: Nonce,
    pub signature: TransactionSignature,
    pub nonce_data_availability_mode: ReservedDataAvailabilityMode,
    pub fee_data_availability_mode: ReservedDataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
    pub version: TransactionVersion,
    pub r#type: InvokeType,
}

/// An invoke transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum ExternalInvokeTransaction {
    V0(ExternalInvokeTransactionV0),
    V1(ExternalInvokeTransactionV1),
    V3(ExternalInvokeTransactionV3),
}
/// A declare transaction of a Cairo-v0 (deprecated) contract class that can be added to Starknet
/// through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeclareTransactionV1 {
    pub contract_class: ContractClass,
    pub sender_address: ContractAddress,
    pub nonce: Nonce,
    pub max_fee: Fee,
    pub version: TransactionVersion,
    pub signature: TransactionSignature,
    pub r#type: DeclareType,
}

/// A declare transaction of a Cairo-v1 contract class that can be added to Starknet through the
/// Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeclareTransactionV2 {
    pub contract_class: ContractClass,
    pub compiled_class_hash: CompiledClassHash,
    pub sender_address: ContractAddress,
    pub nonce: Nonce,
    pub max_fee: Fee,
    pub version: TransactionVersion,
    pub signature: TransactionSignature,
    pub r#type: DeclareType,
}

/// A declare transaction of a Cairo-v1 contract class that can be added to Starknet through the
/// Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
// TODO(shahak): Add tests for declare v3.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExternalDeclareTransactionV3 {
    pub contract_class: ContractClass,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub compiled_class_hash: CompiledClassHash,
    pub sender_address: ContractAddress,
    pub nonce_data_availability_mode: ReservedDataAvailabilityMode,
    pub fee_data_availability_mode: ReservedDataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
    pub version: TransactionVersion,
    pub r#type: DeclareType,
}

/// A declare transaction that can be added to Starknet through the Starknet gateway.
/// It has a serialization format that the Starknet gateway accepts in the `add_transaction`
/// HTTP method.
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum ExternalDeclareTransaction {
    V1(ExternalDeclareTransactionV1),
    V2(ExternalDeclareTransactionV2),
    V3(ExternalDeclareTransactionV3),
}

// The structs that are implemented here are the structs that have deviations from starknet_api.

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct ContractClassV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub abi: Option<Vec<DeprecatedContractClassAbiEntry>>,
    #[serde(rename = "program")]
    // TODO(shahak): Create a struct for a compressed base64 value.
    pub compressed_program: String,
    pub entry_points_by_type: HashMap<DeprecatedEntryPointType, Vec<DeprecatedEntryPoint>>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct ContractClassV1 {
    // TODO(shahak): Create a struct for a compressed base64 value.
    #[serde(rename = "sierra_program")]
    pub compressed_sierra_program: String,
    pub contract_class_version: String,
    pub entry_points_by_type: HashMap<EntryPointType, Vec<EntryPoint>>,
    pub abi: String,
}


#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ContractClass {
    V0(ContractClassV0),
    V1(ContractClassV1),
}
