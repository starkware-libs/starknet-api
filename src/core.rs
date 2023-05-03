#[cfg(test)]
#[path = "core_test.rs"]
mod core_test;

use std::fmt::Debug;

use derive_more::Display;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Serializer, Value};
use starknet_crypto::FieldElement;

use crate::hash::{pedersen_hash_array, StarkFelt, StarkHash};
use crate::transaction::{Calldata, ContractAddressSalt};
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

/// The maximal size of storage var.
pub const MAX_STORAGE_ITEM_SIZE: u16 = 256;
/// The prefix used in the calculation of a contract address.
pub const CONTRACT_ADDRESS_PREFIX: &str = "STARKNET_CONTRACT_ADDRESS";
/// The size of the contract address domain.
pub static CONTRACT_ADDRESS_DOMAIN_SIZE: Lazy<StarkFelt> = Lazy::new(|| {
    StarkFelt::try_from(PATRICIA_KEY_UPPER_BOUND)
        .unwrap_or_else(|_| panic!("Failed to convert {PATRICIA_KEY_UPPER_BOUND} to StarkFelt"))
});
/// The address upper bound; it is defined to be congruent with the storage var address upper bound.
pub static L2_ADDRESS_UPPER_BOUND: Lazy<FieldElement> = Lazy::new(|| {
    FieldElement::from(*CONTRACT_ADDRESS_DOMAIN_SIZE) - FieldElement::from(MAX_STORAGE_ITEM_SIZE)
});

impl TryFrom<StarkHash> for ContractAddress {
    type Error = StarknetApiError;
    fn try_from(hash: StarkHash) -> Result<Self, Self::Error> {
        Ok(Self(PatriciaKey::try_from(hash)?))
    }
}

// TODO: Add a hash_function as a parameter
pub fn calculate_contract_address(
    salt: ContractAddressSalt,
    class_hash: ClassHash,
    constructor_calldata: &Calldata,
    deployer_address: ContractAddress,
) -> Result<ContractAddress, StarknetApiError> {
    let constructor_calldata_hash = pedersen_hash_array(&constructor_calldata.0);
    let contract_address_prefix = format!("0x{}", hex::encode(CONTRACT_ADDRESS_PREFIX));
    let mut address = FieldElement::from(pedersen_hash_array(&[
        StarkFelt::try_from(contract_address_prefix.as_str())?,
        *deployer_address.0.key(),
        salt.0,
        class_hash.0,
        constructor_calldata_hash,
    ]));
    address = address % *L2_ADDRESS_UPPER_BOUND;

    ContractAddress::try_from(StarkFelt::from(address))
}

fn compute_class_hash_from_json(contract_class: &Value) -> String {
    let mut abi_json = json!({
        "abi": contract_class.get("abi").unwrap_or(&Value::Null),
        "program": contract_class.get("program").unwrap_or(&Value::Null)
    });

    let program_json = abi_json.get_mut("program").expect("Program key should be present in the JSON object");
    let debug_info_json = program_json.get_mut("debug_info");
    if debug_info_json.is_some() {
        program_json
            .as_object_mut()
            .expect("Not a JSON object")
            .insert("debug_info".to_owned(), serde_json::Value::Null);
    }

    let mut new_object = serde_json::Map::<String, Value>::new();
    let res = crate::utils::traverse_and_exclude_recursively(
        &abi_json,
        &mut new_object,
        &|key, value| {
            return (key == "attributes" || key == "accessible_scopes")
                && value.is_array()
                && value.as_array().expect("Not a JSON array").is_empty();
        },
    );

    let mut writer = Vec::with_capacity(128);
    let mut serializer =
        Serializer::with_formatter(&mut writer, crate::serde_utils::StarknetFormatter);
    res.serialize(&mut serializer).expect("Unable to serialize with custom formatter");
    let str_json = String::from_utf8(writer).expect("Cant convert to UTF-8 string");

    let keccak_result = crate::hash::sn_keccak(str_json.as_bytes());
    keccak_result
}

fn entry_points_hash_by_type_from_json(
    contract_class: &Value,
    entry_point_type: &str,
) -> StarkFelt {
    let felts = contract_class
        .get("entry_points_by_type")
        .unwrap_or(&serde_json::Value::Null)
        .get(entry_point_type)
        .unwrap_or(&serde_json::Value::Null)
        .as_array()
        .unwrap_or(&Vec::<serde_json::Value>::new())
        .iter()
        .flat_map(|entry| {
            let selector = get_starkfelt_from_json_unsafe(entry, "selector");
            let offset = get_starkfelt_from_json_unsafe(entry, "offset");

            vec![selector, offset]
        })
        .collect::<Vec<StarkFelt>>();

    pedersen_hash_array(&felts)
}

fn get_starkfelt_from_json_unsafe(json: &Value, key: &str) -> StarkFelt {
    StarkFelt::try_from(json.get(key).expect("Key not found").as_str().expect("Not a JSON string"))
        .expect("Not a valid hash")
}

pub fn compute_contract_class_hash_v0(contract_class: &serde_json::Value) -> ClassHash {
    // api version
    let api_version = StarkFelt::try_from(format!("0x{}", hex::encode([0u8])).as_str())
        .expect("Not a valid hash");

    // external entry points hash
    let external_entry_points_hash =
        entry_points_hash_by_type_from_json(contract_class, "EXTERNAL");

    // l1 handler entry points hash
    let l1_entry_points_hash = entry_points_hash_by_type_from_json(contract_class, "L1_HANDLER");

    // constructor handler entry points hash
    let constructor_entry_points_hash =
        entry_points_hash_by_type_from_json(contract_class, "CONSTRUCTOR");

    // builtins hash
    let builtins_encoded = contract_class
        .get("program")
        .unwrap_or(&serde_json::Value::Null)
        .get("builtins")
        .unwrap_or(&serde_json::Value::Null)
        .as_array()
        .unwrap_or(&Vec::<serde_json::Value>::new())
        .iter()
        .map(|str| {
            let hex_str = str
                .as_str()
                .expect("Not a JSON string")
                .as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("");
            format!("0x{}", hex_str)
        })
        .collect::<Vec<String>>();

    let builtins_encoded_as_felts = builtins_encoded
        .iter()
        .map(|s| {
            return StarkFelt::try_from(s.as_str()).expect("Not a valid hash");
        })
        .collect::<Vec<StarkFelt>>();

    let builtins_hash = pedersen_hash_array(&builtins_encoded_as_felts);

    // hinted class hash
    let hinted_class_hash = compute_class_hash_from_json(contract_class);

    // program data hash
    let program_data_felts = contract_class
        .get("program")
        .unwrap_or(&Value::Null)
        .get("data")
        .unwrap_or(&Value::Null)
        .as_array()
        .unwrap_or(&Vec::<Value>::new())
        .iter()
        .map(|str| {
            return StarkFelt::try_from(str.as_str().expect("Not a JSON string"))
                .expect("Not a valid hash");
        })
        .collect::<Vec<StarkFelt>>();

    let program_data_hash = pedersen_hash_array(&program_data_felts);

    ClassHash(pedersen_hash_array(&vec![
        api_version,
        external_entry_points_hash,
        l1_entry_points_hash,
        constructor_entry_points_hash,
        builtins_hash,
        StarkFelt::try_from(hinted_class_hash.as_str()).expect("Not a valid hash"),
        program_data_hash,
    ]))
}

/// The hash of a ContractClass.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
    Display,
)]
pub struct ClassHash(pub StarkHash);

/// The hash of a compiled ContractClass.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
    Display,
)]
pub struct CompiledClassHash(pub StarkHash);

/// A general type for nonces.
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct Nonce(pub StarkFelt);

/// The selector of an [EntryPoint](`crate::deprecated_contract_class::EntryPoint`).
#[derive(
    Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord,
)]
pub struct EntryPointSelector(pub StarkHash);

/// The root of the global state at a [Block](`crate::block::Block`)
/// and [StateUpdate](`crate::state::StateUpdate`).
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Eq,
    PartialEq,
    Hash,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
    Display,
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
        if value < *CONTRACT_ADDRESS_DOMAIN_SIZE {
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
macro_rules! patricia_key {
    ($s:expr) => {
        PatriciaKey::try_from(StarkHash::try_from($s).unwrap()).unwrap()
    };
}
