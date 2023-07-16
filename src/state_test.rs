use std::collections::HashMap;

use serde_json::json;

use crate::deprecated_contract_class::EntryPointOffset;
use crate::hash::StarkFelt;

#[test]
fn entry_point_offset_from_json_str() {
    let data = r#"
        {
            "offset_1":  "0x2" ,
            "offset_2": "0x7b"
        }"#;
    let offsets: HashMap<String, EntryPointOffset> = serde_json::from_str(data).unwrap();

    assert_eq!(EntryPointOffset(StarkFelt::try_from("0x2").unwrap()), offsets["offset_1"]);
    assert_eq!(EntryPointOffset(StarkFelt::try_from("0x7b").unwrap()), offsets["offset_2"]);
}

#[test]
fn entry_point_offset_into_json_str() {
    let offset = EntryPointOffset(StarkFelt::try_from("0x123").unwrap());
    assert_eq!(json!(offset), json!("0x123"));
}
