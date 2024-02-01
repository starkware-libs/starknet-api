//! Cryptographic utilities.
//! This module provides cryptographic utilities.
#[cfg(test)]
#[path = "crypto_test.rs"]
mod crypto_test;

use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;
use starknet_types_core::felt::Felt;

/// An error that can occur during cryptographic operations.
#[derive(thiserror::Error, Clone, Debug)]
pub enum CryptoError {
    #[error("Invalid public key {0:?}.")]
    InvalidPublicKey(PublicKey),
    #[error("Invalid message hash {0:?}.")]
    InvalidMessageHash(Felt),
    #[error("Invalid r {0:?}.")]
    InvalidR(Felt),
    #[error("Invalid s {0:?}.")]
    InvalidS(Felt),
}

/// A public key.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct PublicKey(pub Felt);

/// A signature.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct Signature {
    pub r: Felt,
    pub s: Felt,
}

/// Verifies the authenticity of a signed message hash given the public key of the signer.
pub fn verify_message_hash_signature(
    message_hash: &Felt,
    signature: &Signature,
    public_key: &PublicKey,
) -> Result<bool, CryptoError> {
    starknet_crypto::verify(
        &FieldElement::from_mont(public_key.0.to_raw_reversed()),
        &FieldElement::from_mont(message_hash.to_raw_reversed()),
        &FieldElement::from_mont(signature.r.to_raw_reversed()),
        &FieldElement::from_mont(signature.s.to_raw_reversed()),
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
