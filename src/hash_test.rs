use starknet_types_core::felt::Felt;
use starknet_types_core::hash::{Pedersen, StarkHash};

use crate::transaction::Fee;

#[test]
fn pedersen_hash_correctness() {
    // Test vectors from https://github.com/starkware-libs/crypto-cpp/blob/master/src/starkware/crypto/pedersen_hash_test.cc
    let a = Felt::from_hex("0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb")
        .unwrap();
    let b = Felt::from_hex("0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a")
        .unwrap();
    let expected =
        Felt::from_hex("0x030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662")
            .unwrap();
    assert_eq!(Pedersen::hash(&a, &b), expected);
}

#[test]
fn pedersen_hash_array_correctness() {
    let a = Felt::from(0xaa);
    let b = Felt::from(0xbb);
    let c = Felt::from(0xcc);
    let expected = Pedersen::hash(
        &Pedersen::hash(&Pedersen::hash(&Pedersen::hash(&Felt::from(0x0), &a), &b), &c),
        &Felt::from(0x3),
    );
    assert_eq!(Pedersen::hash_array(&[a, b, c]), expected);
}

#[test]
fn hash_json_serde() {
    let hash = Felt::from(0x123);
    assert_eq!(hash, serde_json::from_str(&serde_json::to_string(&hash).unwrap()).unwrap());
}

#[test]
fn hash_serde() {
    fn enc_len(n_nibbles: usize) -> usize {
        match n_nibbles {
            0..=27 => n_nibbles / 2 + 1,
            28..=33 => 17,
            _ => 32,
        }
    }

    // 64 nibbles are invalid.
    for n_nibbles in 0..64 {
        let mut bytes = [0u8; 32];
        // Set all nibbles to 0xf.
        for i in 0..n_nibbles {
            bytes[31 - (i >> 1)] |= 15 << (4 * (i & 1));
        }
        let h = Felt::from_bytes_be(&bytes);
        let mut res = Vec::new();
        assert!(h.serialize(&mut res).is_ok());
        assert_eq!(res.len(), enc_len(n_nibbles));
        let mut reader = &res[..];
        let d = Felt::deserialize(&mut reader).unwrap();
        assert_eq!(Felt::from_bytes_be(&bytes), d);
    }
}

#[test]
fn fee_to_starkfelt() {
    let fee = Fee(u128::MAX);
    assert_eq!(Felt::from(fee).to_fixed_hex_string(), format!("{:#066x}", fee.0));
}

#[test]
fn felt_to_u64_and_back() {
    // Positive flow.
    let value = u64::MAX;
    let felt: Felt = value.into();
    let new_value: u64 = felt.to_u64().unwrap();
    assert_eq!(value, new_value);

    // Negative flow.
    let value: u128 = u128::from(u64::MAX) + 1;
    let another_felt: Felt = value.into();

    assert!(another_felt.to_u64().is_none());
}
