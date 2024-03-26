use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, EntryPointSelector, Nonce};
use crate::data_availability::DataAvailabilityMode;
use crate::deprecated_contract_class::ContractClass as ContractClassV0;
use crate::state::ContractClass as ContractClassV1;
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, Fee, PaymasterData,
    ResourceBoundsMapping, Tip, TransactionHash, TransactionSignature, TransactionVersion,
};

// A contract class.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ContractClass {
    V0(ContractClassV0),
    V1(ContractClassV1),
}

/// An Internal transaction.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum InternalTransaction {
    /// A declare transaction.
    Declare(InternalDeclareTransaction),
    /// A deploy account transaction.
    DeployAccount(InternalDeployAccountTransaction),
    /// An invoke transaction.
    Invoke(InternalInvokeTransaction),
    /// An L1 handler transaction.
    L1Handler(InternalL1HandlerTransaction),
}

// # Forbid by default query-version transactions.
// only_query: ClassVar[bool] = False

/// Internal Common fields.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalCommonFields {
    pub signature: TransactionSignature,
    pub sender_address: ContractAddress,
    pub only_query: bool,
    pub tx_hash: TransactionHash,
}

/// A declare V0.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InternalDeclareTransactionV0 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub max_fee: Fee,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub contract_class: ContractClass,
    pub abi_length: usize,
}

/// A declare V1.
/// Note that transaction hash calculation is different than in V0.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InternalDeclareTransactionV1 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub max_fee: Fee,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub contract_class: ContractClass,
    pub abi_length: usize,
}

/// A declare V2 transaction.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InternalDeclareTransactionV2 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub max_fee: Fee,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub compiled_class_hash: CompiledClassHash,
    pub contract_class: ContractClass,
    pub sierra_program_length: usize,
    pub abi_length: usize,
}

/// A declare V3 transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InternalDeclareTransactionV3 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub compiled_class_hash: CompiledClassHash,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
    pub contract_class: ContractClass,
    pub sierra_program_length: usize,
    pub abi_length: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum InternalDeclareTransaction {
    V0(InternalDeclareTransactionV0),
    V1(InternalDeclareTransactionV1),
    V2(InternalDeclareTransactionV2),
    V3(InternalDeclareTransactionV3),
}

macro_rules! implement_declare_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(pub fn $field(&self) -> $field_type {
            match self {
                Self::V0(tx) => tx.$field.clone(),
                Self::V1(tx) => tx.$field.clone(),
                Self::V2(tx) => tx.$field.clone(),
                Self::V3(tx) => tx.$field.clone(),
            }
        })*
    };
}

impl InternalDeclareTransaction {
    implement_declare_tx_getters!(
        (class_hash, ClassHash),
        (nonce, Nonce) /* (sender_address, ContractAddress),
                        * (signature, TransactionSignature) */
    );

    pub fn version(&self) -> TransactionVersion {
        match self {
            InternalDeclareTransaction::V0(_) => TransactionVersion::ZERO,
            InternalDeclareTransaction::V1(_) => TransactionVersion::ONE,
            InternalDeclareTransaction::V2(_) => TransactionVersion::TWO,
            InternalDeclareTransaction::V3(_) => TransactionVersion::THREE,
        }
    }
}

/// A deploy account V1 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalDeployAccountTransactionV1 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub max_fee: Fee,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
}

/// A deploy account V3 transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]

pub struct InternalDeployAccountTransactionV3 {
    #[serde(flatten)]
    pub common_fields: InternalCommonFields,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, From)]
pub enum InternalDeployAccountTransaction {
    V1(InternalDeployAccountTransactionV1),
    V3(InternalDeployAccountTransactionV3),
}

macro_rules! implement_deploy_account_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(
            pub fn $field(&self) -> $field_type {
                match self {
                    Self::V1(tx) => tx.$field.clone(),
                    Self::V3(tx) => tx.$field.clone(),
                }
            }
        )*
    };
}

impl InternalDeployAccountTransaction {
    implement_deploy_account_tx_getters!(
        (class_hash, ClassHash),
        (constructor_calldata, Calldata),
        (contract_address_salt, ContractAddressSalt),
        (nonce, Nonce) // (signature, TransactionSignature)
    );

    pub fn version(&self) -> TransactionVersion {
        match self {
            InternalDeployAccountTransaction::V1(_) => TransactionVersion::ONE,
            InternalDeployAccountTransaction::V3(_) => TransactionVersion::THREE,
        }
    }
}
/// An invoke V0 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalInvokeTransactionV0 {
    pub max_fee: Fee,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

/// An invoke V1 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalInvokeTransactionV1 {
    pub max_fee: Fee,
    pub nonce: Nonce,
    pub calldata: Calldata,
}

/// An invoke V3 transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalInvokeTransactionV3 {
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub calldata: Calldata,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord, From)]
pub enum InternalInvokeTransaction {
    V0(InternalInvokeTransactionV0),
    V1(InternalInvokeTransactionV1),
    V3(InternalInvokeTransactionV3),
}

macro_rules! implement_invoke_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(pub fn $field(&self) -> $field_type {
            match self {
                Self::V0(tx) => tx.$field.clone(),
                Self::V1(tx) => tx.$field.clone(),
                Self::V3(tx) => tx.$field.clone(),
            }
        })*
    };
}

impl InternalInvokeTransaction {
    // implement_invoke_tx_getters!((calldata, Calldata), (signature, TransactionSignature));
    implement_invoke_tx_getters!((calldata, Calldata));

    pub fn nonce(&self) -> Nonce {
        match self {
            Self::V0(_) => Nonce::default(),
            Self::V1(tx) => tx.nonce,
            Self::V3(tx) => tx.nonce,
        }
    }

    // pub fn sender_address(&self) -> ContractAddress {
    //     match self {
    //         Self::V0(tx) => tx.contract_address,
    //         Self::V1(tx) => tx.sender_address,
    //         Self::V3(tx) => tx.sender_address,
    //     }
    // }

    pub fn version(&self) -> TransactionVersion {
        match self {
            InternalInvokeTransaction::V0(_) => TransactionVersion::ZERO,
            InternalInvokeTransaction::V1(_) => TransactionVersion::ONE,
            InternalInvokeTransaction::V3(_) => TransactionVersion::THREE,
        }
    }
}

/// An L1 handler transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InternalL1HandlerTransaction {
    pub version: TransactionVersion,
    pub nonce: Nonce,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}
