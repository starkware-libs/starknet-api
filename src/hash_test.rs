use web3::types::U256;

use crate::hash::{pedersen_hash, pedersen_hash_array, starknet_keccak, StarkFelt, MASK};
use crate::stark_felt;

#[test]
fn pedersen_hash_correctness() {
    // Test vectors from https://github.com/starkware-libs/crypto-cpp/blob/master/src/starkware/crypto/pedersen_hash_test.cc
    let a = stark_felt!("0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let b = stark_felt!("0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let expected =
        stark_felt!("0x030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662");
    assert_eq!(pedersen_hash(&a, &b), expected);
}

#[test]
fn pedersen_hash_array_correctness() {
    let a = stark_felt!("0xaa");
    let b = stark_felt!("0xbb");
    let c = stark_felt!("0xcc");
    let expected = pedersen_hash(
        &pedersen_hash(&pedersen_hash(&pedersen_hash(&stark_felt!("0x0"), &a), &b), &c),
        &stark_felt!("0x3"),
    );
    assert_eq!(pedersen_hash_array(&[a, b, c]), expected);
}

#[test]
fn hash_macro() {
    assert_eq!(
        stark_felt!("0x123"),
        StarkFelt::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0x1, 0x23
        ])
        .unwrap()
    );
}

#[test]
fn hash_json_serde() {
    let hash = stark_felt!("0x123");
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
        let h = StarkFelt::new(bytes).unwrap();
        let mut res = Vec::new();
        assert!(h.serialize(&mut res).is_ok());
        assert_eq!(res.len(), enc_len(n_nibbles));
        let mut reader = &res[..];
        let d = StarkFelt::deserialize(&mut reader).unwrap();
        assert_eq!(bytes, d.0);
    }
}

#[test]
fn starknet_keccak_mask() {
    let mask: U256 = U256::pow(U256::from(2), U256::from(250)) - U256::from(1);
    assert_eq!(mask, MASK);
}

#[test]
fn starknet_keccak_calculation() {
    // Test result is taken from <https://www.cairo-lang.org/docs/hello_starknet/intro.html> tutorial.
    // increase_balance function selector can be found in contract_compiled.json ->
    // entry_points_by_type object.
    let expected_keccak_felt =
        stark_felt!("0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320");
    let increase_balance_keccak_felt = starknet_keccak("increase_balance".as_bytes()).unwrap();
    assert_eq!(increase_balance_keccak_felt, expected_keccak_felt);
}
