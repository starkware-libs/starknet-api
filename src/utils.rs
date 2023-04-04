use serde_json::Value;

/// because of the preserve_order feature enabled in the serde_json crate
/// removing a key from the object changes the order of the keys
/// When serde_json is not being used with the preserver order feature
/// deserializing to a serde_json::Value changes the order of the keys
///
/// go through the object by visiting every key and value recursively,
/// and not including them into a new json obj if the condition is met
/// Empty objects are not included
pub fn traverse_and_exclude_recursively<F>(
    value: &Value,
    new_object: &mut serde_json::Map<String, Value>,
    condition: &F,
) -> serde_json::Value
where
    F: Fn(&String, &Value) -> bool,
{
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let mut inner_obj = serde_json::Map::new();

                if condition(key, value) {
                    continue;
                }
                let inner_val = traverse_and_exclude_recursively(value, &mut inner_obj, condition);
                new_object.insert(key.to_string(), inner_val);
            }

            Value::Object(new_object.clone())
        }
        // arrays are visited like the objects - recursively
        Value::Array(array) => {
            let mut inner_arr = Vec::<Value>::new();

            for value in array {
                let mut inner_obj = serde_json::Map::new();
                let inner_val = traverse_and_exclude_recursively(value, &mut inner_obj, condition);

                if !(inner_val.is_object()
                    && inner_val.as_object().expect("Not a json object").is_empty())
                {
                    inner_arr.push(inner_val)
                }
            }

            Value::Array(inner_arr)
        }
        // handle non-object, non-array values
        _ => value.clone(),
    }
}

/// because of the preserve_order feature enabled in the serde_json crate
/// removing a key from the object changes the order of the keys
/// When serde_json is not being used with the preserver order feature
/// deserializing to a serde_json::Value changes the order of the keys
/// Go through object's top level keys and remove those that pass the condition
pub fn traverse_and_exclude_top_level_keys<F>(value: &Value, condition: &F) -> serde_json::Value
where
    F: Fn(&String, &Value) -> bool,
{
    if !value.is_object() {
        return value.clone();
    }

    let mut new_obj = serde_json::Map::new();

    for (key, value) in value.as_object().expect("Not a json object") {
        if condition(key, value) {
            continue;
        }

        new_obj.insert(key.clone(), value.clone());
    }

    Value::Object(new_obj)
}
