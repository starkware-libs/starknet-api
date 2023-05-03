use serde_json::Value;

#[test]
fn serde_remove_elements_from_json() {
    let input = r#"
        {
            "name": "John Doe",
            "isStudent": true,
            "age":30,
            "address": {
                "street": "Vlvo",
                "city": "Anytown",
                "state": "Any"
            },
            "should_be_removed": [],
            "scores": 
            [
                {
                    "street": "AAA",
                    "age": 5,
                    "should_be_removed": []
                },
                {
                    "age": 5
                }
            ],
            "arr": [90, 85, 95]
        }
    "#;
    let expected_output = r#"
        {
            "name": "John Doe",
            "isStudent": true,
            "age":30,
            "address": {
                "street": "Vlvo",
                "city": "Anytown",
                "state": "Any"
            },
            "scores": 
            [
                {
                    "street": "AAA",
                    "age": 5
                },
                {
                    "age": 5
                }
            ],
            "arr": [90, 85, 95]
        }
    "#;
    let value: Value = serde_json::from_str(input).unwrap();
    let mut new_object: serde_json::Map<String, Value> = serde_json::Map::new();

    let res =
        crate::utils::traverse_and_exclude_recursively(&value, &mut new_object, &|key, val| {
            return key == "should_be_removed"
                && val.is_array()
                && val.as_array().unwrap().is_empty();
        });

    assert_eq!(res, serde_json::from_str::<serde_json::Value>(expected_output).unwrap());
}
