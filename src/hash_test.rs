use crate::hash::{pedersen_array_hash, pedersen_hash, StarkHash};
use crate::shash;

#[test]
fn pedersen_hash_correctness() {
    // Test vectors from https://github.com/starkware-libs/crypto-cpp/blob/master/src/starkware/crypto/pedersen_hash_test.cc
    let a = shash!("0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let b = shash!("0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let expected = shash!("0x030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662");
    assert_eq!(pedersen_hash(&a, &b).unwrap(), expected);
}

#[test]
fn pedersen_array_hash_correctness() {
    // Test vectors from https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-core/src/crypto.rs
    let a = shash!("0xaa");
    let b = shash!("0xbb");
    let c = shash!("0xcc");
    let d = shash!("0xdd");
    let expected = shash!("0x025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69");
    assert_eq!(pedersen_array_hash(&[a, b, c, d]).unwrap(), expected);
}

#[test]
fn hash_macro() {
    assert_eq!(
        shash!("0x123"),
        StarkHash::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0x1, 0x23
        ])
        .unwrap()
    );
}

#[test]
fn hash_json_serde() {
    let hash = shash!("0x123");
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
        let h = StarkHash::new(bytes).unwrap();
        let mut res = Vec::new();
        assert!(h.serialize(&mut res).is_ok());
        assert_eq!(res.len(), enc_len(n_nibbles));
        let mut reader = &res[..];
        let d = StarkHash::deserialize(&mut reader).unwrap();
        assert_eq!(bytes, d.0);
    }
}
