use std::collections::HashMap;

use crate::state::EntryPointOffset;

#[test]
fn entry_point_offset_from_str() {
    let offset = EntryPointOffset(123);
    let as_str: String = offset.into();
    assert_eq!("0x7b", as_str);
    assert_eq!(EntryPointOffset::try_from(as_str).unwrap(), offset);
}

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
    let json_str: String = EntryPointOffset(123).into();
    assert_eq!(json_str, "0x7b");
}
