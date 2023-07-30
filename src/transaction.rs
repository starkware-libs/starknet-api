use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::block::{BlockHash, BlockNumber};
use crate::core::{
    ClassHash, CompiledClassHash, ContractAddress, EntryPointSelector, EthAddress, Nonce,
};
use crate::hash::{StarkFelt, StarkHash};
use crate::serde_utils::PrefixedBytesAsHex;

/// A transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum Transaction {
    /// A declare transaction.
    Declare(DeclareTransaction),
    /// A deploy transaction.
    Deploy(DeployTransaction),
    /// A deploy account transaction.
    DeployAccount(DeployAccountTransaction),
    /// An invoke transaction.
    Invoke(InvokeTransaction),
    /// An L1 handler transaction.
    L1Handler(L1HandlerTransaction),
}

/// A transaction output.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum TransactionOutput {
    /// A declare transaction output.
    Declare(DeclareTransactionOutput),
    /// A deploy transaction output.
    Deploy(DeployTransactionOutput),
    /// A deploy account transaction output.
    DeployAccount(DeployAccountTransactionOutput),
    /// An invoke transaction output.
    Invoke(InvokeTransactionOutput),
    /// An L1 handler transaction output.
    L1Handler(L1HandlerTransactionOutput),
}

impl TransactionOutput {
    pub fn actual_fee(&self) -> Fee {
        match self {
            TransactionOutput::Declare(output) => output.actual_fee,
            TransactionOutput::Deploy(output) => output.actual_fee,
            TransactionOutput::DeployAccount(output) => output.actual_fee,
            TransactionOutput::Invoke(output) => output.actual_fee,
            TransactionOutput::L1Handler(output) => output.actual_fee,
        }
    }

    pub fn events(&self) -> &[Event] {
        match self {
            TransactionOutput::Declare(output) => &output.events,
            TransactionOutput::Deploy(output) => &output.events,
            TransactionOutput::DeployAccount(output) => &output.events,
            TransactionOutput::Invoke(output) => &output.events,
            TransactionOutput::L1Handler(output) => &output.events,
        }
    }
}

/// A StorageDomain.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum StorageDomain {
    #[default]
    OnChain,
    OffChain,
}

/// Account parameters.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct AccountParams {
    pub max_fee: Fee,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
}

macro_rules! implement_account_params_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(pub fn $field(&self) -> $field_type {
            self.$field.clone()
        })*
    };
}

impl AccountParams {
    implement_account_params_getters!(
        (max_fee, Fee),
        (signature, TransactionSignature),
        (nonce, Nonce)
    );
}
/// A declare V0 or V1 transaction (same schema but different version).
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeclareTransactionV0V1 {
    pub account_params: AccountParams,
    pub class_hash: ClassHash,
    pub sender_address: ContractAddress,
}

/// A declare V2 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeclareTransactionV2 {
    pub account_params: AccountParams,
    pub class_hash: ClassHash,
    pub compiled_class_hash: CompiledClassHash,
    pub sender_address: ContractAddress,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum DeclareTransaction {
    V0(DeclareTransactionV0V1),
    V1(DeclareTransactionV0V1),
    V2(DeclareTransactionV2),
}

macro_rules! implement_tx_account_params_getters {
    () => {
        pub fn max_fee(&self) -> Fee {
            self.account_params().max_fee
        }

        pub fn signature(&self) -> TransactionSignature {
            self.account_params().signature.clone()
        }

        pub fn nonce(&self) -> Nonce {
            self.account_params().nonce
        }
    };
}

macro_rules! implement_declare_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(
            pub fn $field(&self) -> $field_type {
                match self {
                    Self::V0(tx) => tx.$field.clone(),
                    Self::V1(tx) => tx.$field.clone(),
                    Self::V2(tx) => tx.$field.clone(),
                }
            }
        )*
    };
}

impl DeclareTransaction {
    implement_tx_account_params_getters!();
    implement_declare_tx_getters!(
        (class_hash, ClassHash),
        (account_params, AccountParams),
        (sender_address, ContractAddress)
    );

    pub fn version(&self) -> TransactionVersion {
        match self {
            DeclareTransaction::V0(_) => TransactionVersion(StarkFelt::from(0_u8)),
            DeclareTransaction::V1(_) => TransactionVersion(StarkFelt::from(1_u8)),
            DeclareTransaction::V2(_) => TransactionVersion(StarkFelt::from(2_u8)),
        }
    }
}

/// A deploy account transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeployAccountTransaction {
    pub account_params: AccountParams,
    pub class_hash: ClassHash,
    pub version: TransactionVersion,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
}

macro_rules! implement_deploy_account_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(
            pub fn $field(&self) -> $field_type {
                self.$field.clone()
            }
        )*
    };
}

impl DeployAccountTransaction {
    implement_tx_account_params_getters!();
    implement_deploy_account_tx_getters!(
        (class_hash, ClassHash),
        (version, TransactionVersion),
        (account_params, AccountParams),
        (constructor_calldata, Calldata),
        (contract_address_salt, ContractAddressSalt)
    );
}

/// A deploy transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeployTransaction {
    pub version: TransactionVersion,
    pub class_hash: ClassHash,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
}

/// An invoke V0 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InvokeTransactionV0 {
    pub max_fee: Fee,
    pub signature: TransactionSignature,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

/// An invoke V1 transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InvokeTransactionV1 {
    pub account_params: AccountParams,
    pub sender_address: ContractAddress,
    pub calldata: Calldata,
}

impl InvokeTransactionV1 {
    pub fn account_params(&self) -> AccountParams {
        self.account_params.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord, From)]
pub enum InvokeTransaction {
    V0(InvokeTransactionV0),
    V1(InvokeTransactionV1),
}

macro_rules! implement_invoke_tx_account_params_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(
            pub fn $field(&self) -> $field_type {
                match self {
                    Self::V0(tx) => tx.$field.clone(),
                    Self::V1(tx) => tx.account_params.$field.clone(),
                }
            }
        )*
    };
}

macro_rules! implement_invoke_tx_getters {
    ($(($field:ident, $field_type:ty)),*) => {
        $(
            pub fn $field(&self) -> $field_type {
                match self {
                    Self::V0(tx) => tx.$field.clone(),
                    Self::V1(tx) => tx.$field.clone(),
                }
            }
        )*
    };
}

impl InvokeTransaction {
    implement_invoke_tx_account_params_getters!((max_fee, Fee), (signature, TransactionSignature));
    implement_invoke_tx_getters!((calldata, Calldata));
    pub fn nonce(&self) -> Option<Nonce> {
        match self {
            Self::V0(_) => None,
            Self::V1(tx) => Some(tx.account_params.nonce),
        }
    }
    pub fn sender_address(&self) -> ContractAddress {
        match self {
            Self::V0(tx) => tx.contract_address,
            Self::V1(tx) => tx.sender_address,
        }
    }
    pub fn version(&self) -> TransactionVersion {
        match self {
            InvokeTransaction::V0(_) => TransactionVersion(StarkFelt::from(0_u8)),
            InvokeTransaction::V1(_) => TransactionVersion(StarkFelt::from(1_u8)),
        }
    }
}

/// An L1 handler transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct L1HandlerTransaction {
    pub version: TransactionVersion,
    pub nonce: Nonce,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

/// A declare transaction output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeclareTransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
}

/// A deploy-account transaction output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeployAccountTransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
    pub contract_address: ContractAddress,
}

/// A deploy transaction output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct DeployTransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
    pub contract_address: ContractAddress,
}

/// An invoke transaction output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct InvokeTransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
}

/// An L1 handler transaction output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct L1HandlerTransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
}

/// A transaction receipt.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct TransactionReceipt {
    pub transaction_hash: TransactionHash,
    pub block_hash: BlockHash,
    pub block_number: BlockNumber,
    #[serde(flatten)]
    pub output: TransactionOutput,
}

/// Transaction execution status.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord, Default)]
pub enum TransactionExecutionStatus {
    #[serde(rename = "SUCCEEDED")]
    #[default]
    // Succeeded is the default variant because old versions of Starknet don't have an execution
    // status and every transaction is considered succeeded
    Succeeded,
    #[serde(rename = "REVERTED")]
    Reverted,
}

/// A fee.
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
#[serde(from = "PrefixedBytesAsHex<16_usize>", into = "PrefixedBytesAsHex<16_usize>")]
pub struct Fee(pub u128);

impl From<PrefixedBytesAsHex<16_usize>> for Fee {
    fn from(val: PrefixedBytesAsHex<16_usize>) -> Self {
        Self(u128::from_be_bytes(val.0))
    }
}

impl From<Fee> for PrefixedBytesAsHex<16_usize> {
    fn from(fee: Fee) -> Self {
        Self(fee.0.to_be_bytes())
    }
}

impl From<Fee> for StarkFelt {
    fn from(fee: Fee) -> Self {
        Self::from(fee.0)
    }
}

/// A Resourcs.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum Resource {
    L1Gas,
    L2Gas,
}

/// A ResourceBounds.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct ResourceBounds {
    // Specifies the maximum amount of each resource allowed for usage during the execution.
    pub max_amount: u128,
    // Specifies the maximum price the user is willing to pay for each resource unit.
    pub max_price_per_unit: u128,
}

/// A ResourcesBounds.
#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct ResourcesBounds(pub HashMap<Resource, ResourceBounds>);

impl Ord for ResourcesBounds {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare the lengths of the HashMaps first
        let length_cmp = self.0.len().cmp(&other.0.len());

        if length_cmp != Ordering::Equal {
            return length_cmp;
        }
        // If the lengths are equal, compare the key-value pairs lexicographically
        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some((self_key, self_value)), Some((other_key, other_value))) => {
                    let key_cmp = self_key.cmp(other_key);
                    if key_cmp != Ordering::Equal {
                        return key_cmp;
                    }

                    let value_cmp = self_value.cmp(other_value);
                    if value_cmp != Ordering::Equal {
                        return value_cmp;
                    }
                }
                (None, None) => break,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for ResourcesBounds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ResourcesBounds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.0.clone() {
            key.hash(state);
            value.hash(state);
        }
    }
}

/// The hash of a [Transaction](`crate::transaction::Transaction`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct TransactionHash(pub StarkHash);

impl Display for TransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A contract address salt.
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct ContractAddressSalt(pub StarkHash);

/// A transaction signature.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct TransactionSignature(pub Vec<StarkFelt>);

/// A transaction version.
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct TransactionVersion(pub StarkFelt);

/// The calldata of a transaction.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct Calldata(pub Arc<Vec<StarkFelt>>);

#[macro_export]
macro_rules! calldata {
    ( $( $x:expr ),* ) => {
        Calldata(vec![$($x),*].into())
    };
}

/// An L1 to L2 message.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct MessageToL2 {
    pub from_address: EthAddress,
    pub payload: L1ToL2Payload,
}

/// An L2 to L1 message.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct MessageToL1 {
    pub from_address: ContractAddress,
    pub to_address: EthAddress,
    pub payload: L2ToL1Payload,
}

/// The payload of [`MessageToL2`].
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct L1ToL2Payload(pub Vec<StarkFelt>);

/// The payload of [`MessageToL1`].
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct L2ToL1Payload(pub Vec<StarkFelt>);

/// An event.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct Event {
    pub from_address: ContractAddress,
    #[serde(flatten)]
    pub content: EventContent,
}

/// An event content.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct EventContent {
    pub keys: Vec<EventKey>,
    pub data: EventData,
}

/// An event key.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct EventKey(pub StarkFelt);

/// An event data.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct EventData(pub Vec<StarkFelt>);

/// The index of a transaction in [BlockBody](`crate::block::BlockBody`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct TransactionOffsetInBlock(pub usize);

/// The index of an event in [TransactionOutput](`crate::transaction::TransactionOutput`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct EventIndexInTransactionOutput(pub usize);
