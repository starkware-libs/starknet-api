use assert_matches::assert_matches;
use starknet_crypto::FieldElement;

use crate::core::{
    calculate_contract_address, ClassHash, ContractAddress, PatriciaKey, StarknetApiError,
    CONTRACT_ADDRESS_PREFIX, L2_ADDRESS_UPPER_BOUND,
};
use crate::hash::{pedersen_hash_array, StarkFelt, StarkHash};
use crate::transaction::{Calldata, ContractAddressSalt};
use crate::{patricia_key, stark_felt};

#[test]
fn patricia_key_valid() {
    let hash = stark_felt!("0x123");
    let patricia_key = PatriciaKey::try_from(hash).unwrap();
    assert_eq!(patricia_key.0, hash);
}

#[test]
fn patricia_key_out_of_range() {
    // 2**251
    let hash = stark_felt!("0x800000000000000000000000000000000000000000000000000000000000000");
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
    let salt = ContractAddressSalt(stark_felt!(1337));
    let class_hash = ClassHash(stark_felt!("0x110"));
    let deployer_address = ContractAddress::default();
    let constructor_calldata =
        Calldata(vec![stark_felt!(60), stark_felt!(70), FieldElement::MAX.into()].into());

    let actual_address =
        calculate_contract_address(salt, class_hash, &constructor_calldata, deployer_address)
            .unwrap();

    let constructor_calldata_hash = pedersen_hash_array(&constructor_calldata.0);
    let address = pedersen_hash_array(&[
        StarkFelt::try_from(format!("0x{}", hex::encode(CONTRACT_ADDRESS_PREFIX)).as_str())
            .unwrap(),
        *deployer_address.0.key(),
        salt.0,
        class_hash.0,
        constructor_calldata_hash,
    ]);
    let mod_address = FieldElement::from(address) % *L2_ADDRESS_UPPER_BOUND;
    let expected_address = ContractAddress::try_from(StarkFelt::from(mod_address)).unwrap();

    assert_eq!(actual_address, expected_address);
}

#[test]
fn test_contract_class_hash_generation() {
    let data_str = std::fs::read_to_string("./resources/contract_compiled.json").unwrap();
    let data: serde_json::Value = serde_json::from_str(&data_str).unwrap();
    let expected_class_hash = ClassHash(
        StarkHash::try_from("0x399998c787e0a063c3ac1d2abac084dcbe09954e3b156d53a8c43a02aa27d35")
            .unwrap(),
    );

    let resulted_class_hash = crate::core::compute_contract_class_hash_v0(&data);
    assert_eq!(resulted_class_hash, expected_class_hash);
}
