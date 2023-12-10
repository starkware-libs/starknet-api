use serde_json::json;

use crate::deprecated_contract_class::EntryPointOffset;
use crate::stdlib::collections::HashMap;

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
