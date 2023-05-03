#[cfg(test)]
#[path = "hash_test.rs"]
mod hash_test;

use std::fmt::{Debug, Display};
use std::io::Error;

use serde::{Deserialize, Serialize};
use sha3::Digest;
use starknet_crypto::{pedersen_hash as starknet_crypto_pedersen_hash, FieldElement};
use web3::types::U256;

use crate::serde_utils::{
    bytes_from_hex_str, hex_str_from_bytes, BytesAsHex, NonPrefixedBytesAsHex, PrefixedBytesAsHex,
};
use crate::StarknetApiError;

/// Genesis state hash.
pub const GENESIS_HASH: &str = "0x0";

// Felt encoding constants.
const CHOOSER_FULL: u8 = 15;
const CHOOSER_HALF: u8 = 14;

/// An alias for [`StarkFelt`].
/// The output of the [Pedersen hash](https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#pedersen_hash).
pub type StarkHash = StarkFelt;

/// Computes Pedersen hash using STARK curve on two elements, as defined
/// in <https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#pedersen_hash.>
pub fn pedersen_hash(felt0: &StarkFelt, felt1: &StarkFelt) -> StarkHash {
    StarkFelt::from(starknet_crypto_pedersen_hash(
        &FieldElement::from(*felt0),
        &FieldElement::from(*felt1),
    ))
}

/// Computes Pedersen hash using STARK curve on an array of elements, as defined
/// in <https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#array_hashing.>
pub fn pedersen_hash_array(felts: &[StarkFelt]) -> StarkHash {
    let current_hash = felts
        .iter()
        .fold(StarkFelt::from(0), |current_hash, felt| pedersen_hash(&current_hash, felt));
    let data_len = StarkFelt::from(felts.len() as u64);
    pedersen_hash(&current_hash, &data_len)
}

/// Starknet Keccak Hash
pub fn sn_keccak(data: &[u8]) -> String {
    let keccak256 = sha3::Keccak256::digest(data);
    let number = U256::from_big_endian(keccak256.as_slice());
    let mask = U256::pow(U256::from(2), U256::from(250)) - U256::from(1);
    let masked_number = number & mask;
    let mut res_bytes: [u8; 32] = [0; 32];
    masked_number.to_big_endian(&mut res_bytes);
    format!("0x{}", hex::encode(res_bytes).trim_start_matches('0'))
}

// TODO: Move to a different crate.
/// The StarkNet [field element](https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#domain_and_range).
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Deserialize, Serialize, PartialOrd, Ord)]
#[serde(try_from = "PrefixedBytesAsHex<32_usize>", into = "PrefixedBytesAsHex<32_usize>")]
pub struct StarkFelt([u8; 32]);

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct StarkFeltAsDecimal(U256);

impl Serialize for StarkFeltAsDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl Display for StarkFeltAsDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl StarkFelt {
    /// Returns a new [`StarkFelt`].
    pub fn new(bytes: [u8; 32]) -> Result<StarkFelt, StarknetApiError> {
        // msb nibble must be 0. This is not a tight bound.
        if bytes[0] < 0x10 {
            return Ok(Self(bytes));
        }
        Err(StarknetApiError::OutOfRange { string: hex_str_from_bytes::<32, true>(bytes) })
    }

    /// Storage efficient serialization for field elements.
    pub fn serialize(&self, res: &mut impl std::io::Write) -> Result<(), Error> {
        // We use the fact that bytes[0] < 0x10 and encode the size of the felt in the 4 most
        // significant bits of the serialization, which we call `chooser`. We assume that 128 bit
        // felts are prevalent (because of how uint256 is encoded in felts).

        // The first i for which nibbles 2i+1, 2i+2 are nonzero. Note that the first nibble is
        // always 0.
        let mut first_index = 31;
        for i in 0..32 {
            let value = self.0[i];
            if value == 0 {
                continue;
            } else if value < 16 {
                // Can encode the chooser and the value on a single byte.
                first_index = i;
            } else {
                // The chooser is encoded with the first nibble of the value.
                first_index = i - 1;
            }
            break;
        }
        let chooser = if first_index < 15 {
            // For 34 up to 63 nibble felts: chooser == 15, serialize using 32 bytes.
            first_index = 0;
            CHOOSER_FULL
        } else if first_index < 18 {
            // For 28 up to 33 nibble felts: chooser == 14, serialize using 17 bytes.
            first_index = 15;
            CHOOSER_HALF
        } else {
            // For up to 27 nibble felts: serialize the lower 1 + (chooser * 2) nibbles of the felt
            // using chooser + 1 bytes.
            (31 - first_index) as u8
        };
        res.write_all(&[(chooser << 4) | self.0[first_index]])?;
        res.write_all(&self.0[first_index + 1..])?;
        Ok(())
    }

    /// Storage efficient deserialization for field elements.
    pub fn deserialize(bytes: &mut impl std::io::Read) -> Option<Self> {
        let mut res = [0u8; 32];

        bytes.read_exact(&mut res[..1]).ok()?;
        let first = res[0];
        let chooser: u8 = first >> 4;
        let first = first & 0x0f;

        let first_index = if chooser == CHOOSER_FULL {
            0
        } else if chooser == CHOOSER_HALF {
            15
        } else {
            (31 - chooser) as usize
        };
        res[0] = 0;
        res[first_index] = first;
        bytes.read_exact(&mut res[first_index + 1..]).ok()?;
        Some(Self(res))
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn str_format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("0x{}", hex::encode(self.0));
        f.debug_tuple("StarkFelt").field(&s).finish()
    }
}

impl TryFrom<PrefixedBytesAsHex<32_usize>> for StarkFelt {
    type Error = StarknetApiError;
    fn try_from(val: PrefixedBytesAsHex<32_usize>) -> Result<Self, Self::Error> {
        StarkFelt::new(val.0)
    }
}

impl TryFrom<&str> for StarkFelt {
    type Error = StarknetApiError;
    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let val = val.trim_start_matches("0x");
        let bytes = bytes_from_hex_str::<32, false>(val)?;
        Self::new(bytes)
    }
}

impl From<u64> for StarkFelt {
    fn from(val: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[24..32].copy_from_slice(&val.to_be_bytes());
        Self(bytes)
    }
}

impl From<FieldElement> for StarkFelt {
    fn from(fe: FieldElement) -> Self {
        // Should not fail.
        Self::new(fe.to_bytes_be()).expect("Convert FieldElement to StarkFelt.")
    }
}

impl From<StarkFelt> for FieldElement {
    fn from(felt: StarkFelt) -> Self {
        // Should not fail.
        Self::from_bytes_be(&felt.0).expect("Convert StarkFelf to FieldElement.")
    }
}

// TODO: Remove once Starknet sequencer returns the global root hash as a hex string with a
// "0x" prefix.
impl TryFrom<NonPrefixedBytesAsHex<32_usize>> for StarkFelt {
    type Error = StarknetApiError;
    fn try_from(val: NonPrefixedBytesAsHex<32_usize>) -> Result<Self, Self::Error> {
        StarkFelt::new(val.0)
    }
}

impl From<StarkFelt> for PrefixedBytesAsHex<32_usize> {
    fn from(felt: StarkFelt) -> Self {
        BytesAsHex(felt.0)
    }
}

impl TryFrom<StarkFelt> for usize {
    type Error = StarknetApiError;
    fn try_from(felt: StarkFelt) -> Result<Self, Self::Error> {
        const COMPLIMENT_OF_USIZE: usize =
            std::mem::size_of::<StarkFelt>() - std::mem::size_of::<usize>();

        let (rest, usize_bytes) = felt.bytes().split_at(COMPLIMENT_OF_USIZE);
        if rest != [0u8; COMPLIMENT_OF_USIZE] {
            return Err(StarknetApiError::OutOfRange { string: felt.to_string() });
        }

        Ok(usize::from_be_bytes(
            usize_bytes.try_into().expect("usize_bytes should be of size usize."),
        ))
    }
}

impl From<StarkFelt> for U256 {
    fn from(felt: StarkFelt) -> Self {
        web3::types::U256::from_big_endian(&felt.0)
    }
}

impl From<StarkFelt> for StarkFeltAsDecimal {
    fn from(felt: StarkFelt) -> Self {
        StarkFeltAsDecimal(U256::from(felt))
    }
}

impl Debug for StarkFelt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str_format(f)
    }
}

impl Display for StarkFelt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// A utility macro to create a [`StarkFelt`] from a hex string representation.
#[cfg(any(feature = "testing", test))]
#[macro_export]
macro_rules! stark_felt {
    ($s:expr) => {
        StarkFelt::try_from($s).unwrap()
    };
}
