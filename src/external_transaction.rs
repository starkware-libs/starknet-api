#[cfg(test)]
#[path = "external_transaction_test.rs"]
mod external_transaction_test;

use serde::{Deserialize, Serialize};

use crate::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce};
use crate::data_availability::DataAvailabilityMode;
use crate::hash::StarkFelt;
use crate::state::EntryPoint;
use crate::transaction::{
    AccountDeploymentData, Calldata, ContractAddressSalt, PaymasterData, ResourceBounds, Tip,
    TransactionSignature,
};

/// Transactions that are ready to be broadcasted to the network through RPC and are not included in
/// a block.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ExternalTransaction {
    #[serde(rename = "DECLARE")]
    Declare(ExternalDeclareTransaction),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(ExternalDeployAccountTransaction),
    #[serde(rename = "INVOKE")]
    Invoke(ExternalInvokeTransaction),
}

macro_rules! implement_ref_getters {
    ($(($member_name:ident, $member_type:ty)), *) => {
        $(pub fn $member_name(&self) -> &$member_type {
            match self {
                ExternalTransaction::Declare(
                    ExternalDeclareTransaction::V3(tx)
                ) => &tx.$member_name,
                ExternalTransaction::DeployAccount(
                    ExternalDeployAccountTransaction::V3(tx)
                ) => &tx.$member_name,
                ExternalTransaction::Invoke(
                    ExternalInvokeTransaction::V3(tx)
                ) => &tx.$member_name
            }
        })*
    };
}

impl ExternalTransaction {
    implement_ref_getters!(
        (resource_bounds, ResourceBoundsMapping),
        (signature, TransactionSignature)
    );
}

/// A RPC declare transaction.
///
/// This transaction is equivalent to the component DECLARE_TXN in the
/// [`Starknet specs`] with a contract class (DECLARE_TXN allows having
/// either a contract class or a class hash).
///
/// [`Starknet specs`]: https://github.com/starkware-libs/starknet-specs/blob/master/api/starknet_api_openrpc.json
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "version")]
pub enum ExternalDeclareTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalDeclareTransactionV3),
}

/// A RPC deploy account transaction.
///
/// This transaction is equivalent to the component DEPLOY_ACCOUNT_TXN in the
/// [`Starknet specs`].
///
/// [`Starknet specs`]: https://github.com/starkware-libs/starknet-specs/blob/master/api/starknet_api_openrpc.json
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "version")]
pub enum ExternalDeployAccountTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalDeployAccountTransactionV3),
}

/// A RPC invoke transaction.
///
/// This transaction is equivalent to the component INVOKE_TXN in the
/// [`Starknet specs`].
///
/// [`Starknet specs`]: https://github.com/starkware-libs/starknet-specs/blob/master/api/starknet_api_openrpc.json
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "version")]
pub enum ExternalInvokeTransaction {
    #[serde(rename = "0x3")]
    V3(ExternalInvokeTransactionV3),
}

/// A declare transaction of a Cairo-v1 contract class that can be added to Starknet through the
/// RPC.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExternalDeclareTransactionV3 {
    // TODO: Check with Shahak why we need to keep the DeclareType.
    // pub r#type: DeclareType,
    pub sender_address: ContractAddress,
    pub compiled_class_hash: CompiledClassHash,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub contract_class: ContractClass,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// A deploy account transaction that can be added to Starknet through the RPC.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExternalDeployAccountTransactionV3 {
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub class_hash: ClassHash,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub paymaster_data: PaymasterData,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// An invoke account transaction that can be added to Starknet through the RPC.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExternalInvokeTransactionV3 {
    pub sender_address: ContractAddress,
    pub calldata: Calldata,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: Tip,
    pub paymaster_data: PaymasterData,
    pub account_deployment_data: AccountDeploymentData,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
}

// The contract class in SN_API state doesn't have `contract_class_version`, not following the spec.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractClass {
    pub sierra_program: Vec<StarkFelt>,
    pub contract_class_version: String,
    pub entry_points_by_type: EntryPointByType,
    pub abi: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EntryPointByType {
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<EntryPoint>,
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<EntryPoint>,
    #[serde(rename = "L1_HANDLER")]
    pub l1handler: Vec<EntryPoint>,
}

// The serialization of the struct in transaction is in capital letters, not following the spec.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ResourceBoundsMapping {
    pub l1_gas: ResourceBounds,
    pub l2_gas: ResourceBounds,
}
