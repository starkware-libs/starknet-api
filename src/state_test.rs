use crate::deprecated_contract_class::EntryPointOffset;
use crate::stdlib::string::String;

#[test]
fn entry_point_offset_from_str() {
    let offset = EntryPointOffset(123);
    let as_str: String = offset.into();
    assert_eq!("0x7b", as_str);
    assert_eq!(EntryPointOffset::try_from(as_str).unwrap(), offset);
}
