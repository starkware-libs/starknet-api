use assert_matches::assert_matches;

use crate::core::{PatriciaKey, StarknetApiError};
use crate::hash::{StarkFelt, StarkHash};
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
