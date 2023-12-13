use std::collections::HashMap;

use serde_json::json;

use crate::deprecated_contract_class::EntryPointOffset;
use crate::state::StorageKey;

#[test]
fn entry_point_offset_from_json_str() {
    let data = r#"
        {
            "offset_1":  2 ,
            "offset_2": "0x7b"
        }"#;
    let offsets: HashMap<String, EntryPointOffset> = serde_json::from_str(data).unwrap();

    assert_eq!(EntryPointOffset(2), offsets["offset_1"]);
    assert_eq!(EntryPointOffset(123), offsets["offset_2"]);
}

#[test]
fn entry_point_offset_into_json_str() {
    let offset = EntryPointOffset(123);
    assert_eq!(json!(offset), json!(format!("{:#x}", offset.0)));
}

#[test]
fn offset_storage_key_add_rhs_ok() {
    let key = StorageKey::from(123u128);
    let offset = -23;
    let expected = StorageKey::from(100u128);
    let result: StorageKey = key + offset;
    assert_eq!(expected, result);
}

#[test]
#[should_panic(expected = "attempt to add to storage key with overflow")]
fn offset_storage_key_add_rhs_err() {
    let key = StorageKey::from(123u128);
    let offset = -124;
    let _: StorageKey = key + offset;
}

#[test]
fn offset_storage_key_add_lhs_ok() {
    let key = StorageKey::from(123u128);
    let offset = -23;
    let expected = StorageKey::from(100u128);
    let result: StorageKey = offset + key;
    assert_eq!(expected, result);
}

#[test]
#[should_panic(expected = "attempt to add to storage key with overflow")]
fn offset_storage_key_add_lhs_err() {
    let key = StorageKey::from(123u128);
    let offset = -124;
    let _: StorageKey = offset + key;
}
