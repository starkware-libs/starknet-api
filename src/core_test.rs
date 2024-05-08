use assert_matches::assert_matches;
use starknet_types_core::felt::Felt;
use starknet_types_core::hash::{Pedersen, StarkHash as CoreStarkHash};

use crate::core::{
    calculate_contract_address, ClassHash, ContractAddress, EthAddress, Nonce, PatriciaKey,
    StarknetApiError, CONTRACT_ADDRESS_PREFIX, L2_ADDRESS_UPPER_BOUND,
};

use crate::transaction::{Calldata, ContractAddressSalt};
use crate::{class_hash, patricia_key, StarkHash};

#[test]
fn patricia_key_valid() {
    let hash = Felt::from_hex_unchecked("0x123");
    let patricia_key = PatriciaKey::try_from(hash).unwrap();
    assert_eq!(patricia_key.0, hash);
}

#[test]
fn patricia_key_out_of_range() {
    // 2**251
    let hash = Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000000000000000000");
    let err = PatriciaKey::try_from(hash);
    assert_matches!(err, Err(StarknetApiError::OutOfRange { string: _err_str }));
}

#[test]
fn patricia_key_macro() {
    assert_eq!(
        patricia_key!("0x123"),
        PatriciaKey::try_from(
            StarkHash::new([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0x1, 0x23
            ])
            .unwrap()
        )
        .unwrap()
    );
}

#[test]
fn test_calculate_contract_address() {
    let salt = ContractAddressSalt(Felt::from(1337_u16));
    let class_hash = class_hash!("0x110");
    let deployer_address = ContractAddress::default();
    let constructor_calldata =
        Calldata(vec![Felt::from(60_u16), Felt::from(70_u16), Felt::MAX.into()].into());

    let actual_address =
        calculate_contract_address(salt, class_hash, &constructor_calldata, deployer_address)
            .unwrap();

    let constructor_calldata_hash = Pedersen::hash_array(&constructor_calldata.0);
    let address = Pedersen::hash_array(&[
        Felt::from_hex(format!("0x{}", hex::encode(CONTRACT_ADDRESS_PREFIX)).as_str())
            .unwrap(),
        *deployer_address.0.key(),
        salt.0,
        class_hash.0,
        constructor_calldata_hash,
    ]);
    let mod_address = address % *L2_ADDRESS_UPPER_BOUND;
    let expected_address = ContractAddress::try_from(Felt::from(mod_address)).unwrap();

    assert_eq!(actual_address, expected_address);
}

#[test]
fn eth_address_serde() {
    let eth_address = EthAddress::try_from(Felt::from_hex_unchecked("0x001")).unwrap();
    let serialized = serde_json::to_string(&eth_address).unwrap();
    assert_eq!(serialized, r#""0x1""#);

    let restored = serde_json::from_str::<EthAddress>(&serialized).unwrap();
    assert_eq!(restored, eth_address);
}

#[test]
fn nonce_overflow() {
    // Increment on this value should overflow back to 0.
    let max_nonce = Nonce(Felt::from(Felt::MAX));

    let overflowed_nonce = max_nonce.try_increment();
    assert_matches!(overflowed_nonce, Err(StarknetApiError::OutOfRange { string: _err_str }));
}
