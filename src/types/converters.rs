use crate::types::{FieldValue, Payload};
use io::Write;
use std::fs::File;
use std::io;

const FIELD_GAP: &str = "    ";

pub struct GoConverter;
pub struct RustConverter;

pub trait Converter {
    fn convert_payloads(payloads: Vec<Payload>);
}

impl Converter for GoConverter {
    fn convert_payloads(payloads: Vec<Payload>) {
        println!("Converting payloads to Go");
    }
}

impl Converter for RustConverter {
    fn convert_payloads(payloads: Vec<Payload>) {
        let mut payload_data = String::new();

        add_imports(&mut payload_data);

        for payload in payloads {
            add_struct_enrichment(&mut payload_data, payload.name.as_str());

            for (field_name, field_value) in payload.data.iter() {
                add_field_enrichment(&mut payload_data, field_name, field_value);
            }

            add_payload_end(&mut payload_data);
            // TODO add the scaffolding for the necessary impl blocks
            // add_impl_enrichments(&mut payload_data);
        }

        write_to_file(payload_data);
    }
}

fn add_imports(payload_data: &mut String) {
    payload_data.push_str("use serde::{Deserialize, Serialize};\n\n");
}

fn add_struct_enrichment(payload_data: &mut String, struct_name: &str) {
    let derive_block = "#[derive(Deserialize, Serialize)]\n";
    payload_data.push_str(derive_block);
    let struct_def = format!("struct {} {{\n", struct_name);
    payload_data.push_str(&struct_def);
}

fn add_field_enrichment(payload_data: &mut String, field_name: &String, field_value: &FieldValue) {
    if let Some(description) = field_value.get_description() {
        let field_comment = format!("{}/// {}\n", FIELD_GAP, description);
        payload_data.push_str(&field_comment);
    }

    let field_data = format!(
        "{}pub {}: {},\n",
        FIELD_GAP,
        field_name,
        field_value.get_value()
    );
    payload_data.push_str(&field_data);
}

fn add_payload_end(payload_data: &mut String) {
    let struct_end = "}\n\n";
    payload_data.push_str(&struct_end);
}

fn write_to_file(payload_data: String) {
    let mut new_file = File::create("./src/generated/payload.rs").expect("Failed to create file");
    Write::write_all(&mut new_file, payload_data.as_bytes()).expect("Failed to write to file");
}
