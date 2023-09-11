//! Cryptographic utilities.
//! This module provides cryptographic utilities.
#[cfg(test)]
#[path = "crypto_test.rs"]
mod crypto_test;

use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;

use crate::hash::StarkFelt;

/// An error that can occur during cryptographic operations.
#[derive(thiserror::Error, Clone, Debug)]
pub enum CryptoError {
    #[error("Invalid public key {0:?}.")]
    InvalidPublicKey(PublicKey),
    #[error("Invalid message hash {0:?}.")]
    InvalidMessageHash(StarkFelt),
    #[error("Invalid r {0:?}.")]
    InvalidR(StarkFelt),
    #[error("Invalid s {0:?}.")]
    InvalidS(StarkFelt),
}

/// A public key.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct PublicKey(pub StarkFelt);

/// A signature.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct Signature {
    pub r: StarkFelt,
    pub s: StarkFelt,
}

/// Verifies the authenticity of a signed message hash given the public key of the signer.
pub fn verify_message_hash_signature(
    message_hash: &StarkFelt,
    signature: &Signature,
    public_key: &PublicKey,
) -> Result<bool, CryptoError> {
    starknet_crypto::verify(
        &public_key.0.into(),
        &FieldElement::from(*message_hash),
        &signature.r.into(),
        &signature.s.into(),
    )
    .map_err(|err| match err {
        starknet_crypto::VerifyError::InvalidPublicKey => {
            CryptoError::InvalidPublicKey(*public_key)
        }
        starknet_crypto::VerifyError::InvalidMessageHash => {
            CryptoError::InvalidMessageHash(*message_hash)
        }
        starknet_crypto::VerifyError::InvalidR => CryptoError::InvalidR(signature.r),
        starknet_crypto::VerifyError::InvalidS => CryptoError::InvalidS(signature.s),
    })
}
