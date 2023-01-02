#[cfg(test)]
#[path = "core_test.rs"]
mod core_test;

use std::fmt::Debug;

use derive_more::Display;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;

use crate::hash::{pedersen_hash_array, StarkFelt, StarkHash};
use crate::transaction::Calldata;
use crate::StarknetApiError;

/// A chain id.
#[derive(Clone, Debug, Display, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct ChainId(pub String);

impl ChainId {
    pub fn as_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.0))
    }
}

/// The address of a contract, used for example in [StateDiff](`crate::state::StateDiff`),
/// [DeclareTransaction](`crate::transaction::DeclareTransaction`), and
/// [BlockHeader](`crate::block::BlockHeader`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct ContractAddress(pub PatriciaKey);

pub const MAX_STORAGE_ITEM_SIZE: u32 = 256;
pub static CONTRACT_ADDRESS_PREFIX: &str = "STARKNET_CONTRACT_ADDRESS";
pub static L2_ADDRESS_UPPER_BOUND: Lazy<FieldElement> = Lazy::new(|| {
    FieldElement::from_hex_be(PATRICIA_KEY_UPPER_BOUND)
        .unwrap_or_else(|_| panic!("Convert {} to FieldElement", PATRICIA_KEY_UPPER_BOUND))
        - MAX_STORAGE_ITEM_SIZE.into()
});

impl TryFrom<StarkHash> for ContractAddress {
    type Error = StarknetApiError;
    fn try_from(hash: StarkHash) -> Result<Self, Self::Error> {
        Ok(Self(PatriciaKey::try_from(hash)?))
    }
}

// TODO(Noa, 01/02/23): Add a hash_function as a parameter
// TODO(Noa, 08/01/23): Add a unit test
pub fn calculate_contract_address(
    salt: StarkFelt,
    class_hash: ClassHash,
    constructor_calldata: &Calldata,
    deployer_address: ContractAddress,
) -> Result<ContractAddress, StarknetApiError> {
    let constructor_calldata_hash = pedersen_hash_array(&constructor_calldata.0);
    let contract_address_prefix_hex = format!("0x{}", hex::encode(CONTRACT_ADDRESS_PREFIX));
    let raw_address = pedersen_hash_array(&[
        StarkFelt::try_from(contract_address_prefix_hex.as_str())?,
        *deployer_address.0.key(),
        salt,
        class_hash.0,
        constructor_calldata_hash,
    ]);
    let mod_raw_address = FieldElement::from(raw_address) % *L2_ADDRESS_UPPER_BOUND;

    ContractAddress::try_from(StarkFelt::from(mod_raw_address))
}

/// The hash of a [ContractClass](`crate::state::ContractClass`).
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct ClassHash(pub StarkHash);

/// A general type for nonces.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct Nonce(pub StarkFelt);

/// The selector of an [EntryPoint](`crate::state::EntryPoint`).
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct EntryPointSelector(pub StarkHash);

/// The root of the global state at a [Block](`crate::block::Block`)
/// and [StateUpdate](`crate::state::StateUpdate`).
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct GlobalRoot(pub StarkHash);

/// A key for nodes of a Patricia tree.
// Invariant: key is in range.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct PatriciaKey(StarkHash);

// 2**251
pub const PATRICIA_KEY_UPPER_BOUND: &str =
    "0x800000000000000000000000000000000000000000000000000000000000000";

impl PatriciaKey {
    pub fn key(&self) -> &StarkHash {
        &self.0
    }
}

impl TryFrom<StarkHash> for PatriciaKey {
    type Error = StarknetApiError;

    fn try_from(value: StarkHash) -> Result<Self, Self::Error> {
        if value < StarkHash::try_from(PATRICIA_KEY_UPPER_BOUND)? {
            return Ok(PatriciaKey(value));
        }
        Err(StarknetApiError::OutOfRange { string: format!("[0x0, {PATRICIA_KEY_UPPER_BOUND})") })
    }
}

impl Debug for PatriciaKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PatriciaKey").field(&self.0).finish()
    }
}

/// A utility macro to create a [`PatriciaKey`] from a hex string representation.
#[cfg(any(feature = "testing", test))]
#[macro_export]
macro_rules! patky {
    ($s:expr) => {
        PatriciaKey::try_from(StarkHash::try_from($s).unwrap()).unwrap()
    };
}
