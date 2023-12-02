//! Representations of canonical [`Starknet`] components.
//!
//! [`Starknet`]: https://starknet.io/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(features = "std"))]
extern crate alloc;

#[cfg(not(features = "std"))]
extern crate thiserror_no_std as thiserror;

pub mod block;
pub mod core;
pub mod data_availability;
pub mod deprecated_contract_class;
pub mod hash;
pub mod serde_utils;
pub mod state;
pub mod transaction;
pub mod type_utils;

mod api_error {

    cfg_if::cfg_if! {
        if #[cfg(feature = "std")] {
            use std::num;
        } else {
            use core::num;
            use alloc::string::String;
        }
    }

    use crate::serde_utils::InnerDeserializationError;

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
        ParseIntError(#[from] num::ParseIntError),
        /// Missing resource type / duplicated resource type.
        #[error("Missing resource type / duplicated resource type; got {0}.")]
        InvalidResourceMappingInitializer(String),
    }
}

pub use api_error::*;
