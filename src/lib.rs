//! Representations of canonical [`Starknet`] components.
//!
//! [`Starknet`]: https://starknet.io/

pub mod block;
pub mod block_hash;
pub mod core;
pub mod crypto;
pub mod data_availability;
pub mod deprecated_contract_class;
pub mod external_transaction;
// pub mod hash;
pub mod internal_transaction;
pub mod serde_utils;
pub mod state;
pub mod transaction;
pub mod transaction_hash;
pub mod type_utils;

use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use std::num::ParseIntError;

use serde_utils::InnerDeserializationError;

/// The error type returned by StarknetApi.
#[derive(thiserror::Error, Clone, Debug)]
pub enum StarknetApiError {
    /// Error in the inner deserialization of the node.
    #[error(transparent)]
    InnerDeserialization(#[from] InnerDeserializationError),
    #[error("Out of range {string}.")]
    /// An error for when a value is out of range.
    OutOfRange { string: String },
    /// Error when serializing into number.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// Missing resource type / duplicated resource type.
    #[error("Missing resource type / duplicated resource type; got {0}.")]
    InvalidResourceMappingInitializer(String),
}

// TODO: solve name conflict with StarkHash from types-rs
pub type StarkHash = Felt;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct PoseidonHash(pub Felt);
