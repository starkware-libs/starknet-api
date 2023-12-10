//! Representations of canonical [`Starknet`] components.
//!
//! [`Starknet`]: https://starknet.io/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(not(feature = "std"))]
include!("./without_std.rs");

pub mod block;
pub mod core;
pub mod data_availability;
pub mod deprecated_contract_class;
pub mod hash;
pub mod serde_utils;
pub mod state;
pub mod transaction;
pub mod type_utils;

pub mod stdlib {
    pub mod collections {
        #[cfg(feature = "std")]
        pub use crate::with_std::collections::*;
        #[cfg(not(feature = "std"))]
        pub use crate::without_std::collections::*;
    }

    #[cfg(feature = "std")]
    pub use crate::with_std::*;
    #[cfg(not(feature = "std"))]
    pub use crate::without_std::*;
}

mod api_error {
    use thiserror_no_std::Error;

    use crate::serde_utils::InnerDeserializationError;
    use crate::stdlib::num;
    use crate::stdlib::string::String;

    /// The error type returned by StarknetApi.
    #[derive(Error, Clone, Debug)]
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
