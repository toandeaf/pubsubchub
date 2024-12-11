use std::collections::HashMap;
use std::fs::File;
use serde_yaml::{from_reader, Value};
use crate::types::FieldValue;

#[derive(Debug)]
pub struct Payload {
    pub name: String,
    pub data: HashMap<String, FieldValue>,
}

pub fn extract_payload_data(config_file: File) -> Vec<Payload> {
    let mut payloads: Vec<Payload> = vec![];
    let data: HashMap<String, Value> = from_reader(config_file).expect("Failed to deserialize");

    for (name, value) in data.iter() {
        if let Value::Mapping(mapping) = value {
            let mut data: HashMap<String, FieldValue> = HashMap::new();

            for (field_name, value) in mapping.iter() {
                if let Value::Mapping(mapping) = value {
                    let field_value: FieldValue = mapping.clone().into();

                    if let Value::String(field_name_string) = field_name {
                        data.insert(field_name_string.to_string(), field_value);
                    }
                }
            }

            let payload = Payload {
                name: name.to_string(),
                data,
            };

            payloads.push(payload);
        }
    }

    payloads
}
