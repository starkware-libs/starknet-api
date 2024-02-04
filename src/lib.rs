//! Representations of canonical [`Starknet`] components.
//!
//! [`Starknet`]: https://starknet.io/

pub mod block;
pub mod core;
pub mod crypto;
pub mod data_availability;
pub mod deprecated_contract_class;
pub mod hash;
pub mod serde_utils;
pub mod state;
pub mod transaction;
pub mod type_utils;

use std::num::ParseIntError;

use serde_utils::InnerDeserializationError;

/// The error type returned by StarknetApi.
#[derive(thiserror::Error, Clone, Debug)]
pub enum StarknetApiError {
    /// Error in the inner deserialization of the node.
    #[error(transparent)]
    InnerDeserialization(#[from] InnerDeserializationError),
    /// An error for when a value is out of range.
    #[error(transparent)]
    OutOfRange(OutOfRangeError),
    /// Error when serializing into number.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// Missing resource type / duplicated resource type.
    #[error("Missing resource type / duplicated resource type; got {0}.")]
    InvalidResourceMappingInitializer(String),
}

#[derive(thiserror::Error, Clone, Debug)]
#[error("Out of range {string}.")]
pub struct OutOfRangeError {
    string: String,
}

impl From<OutOfRangeError> for StarknetApiError {
    fn from(error: OutOfRangeError) -> Self {
        Self::OutOfRange(error)
    }
}
