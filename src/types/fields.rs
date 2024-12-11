use serde_yaml::Mapping;
use crate::types::floats::FloatFormat;
use crate::types::ints::IntFormat;
use crate::types::strings::StringFormat;

#[derive(Debug)]
pub enum FieldValue {
    String(StringFormat, bool, Option<String>),
    Int(IntFormat, bool, Option<String>),
    Float(FloatFormat, bool, Option<String>),
    Boolean(bool, Option<String>),
}

impl FieldValue {
    pub fn get_value(&self) -> String {
        let mut string_builder = String::new();

        let value = match self {
            FieldValue::String(format, _, _) => match format {
                StringFormat::Date | StringFormat::DateTime => "chrono::NaiveDate",
                StringFormat::Email | StringFormat::Default => "String",
            },
            FieldValue::Int(format, _, _) => match format {
                IntFormat::Uint8 | IntFormat::Int8 => "i8",
                IntFormat::Uint16 | IntFormat::Int16 => "i16",
                IntFormat::Uint32 | IntFormat::Int32 => "i32",
                IntFormat::Uint64 | IntFormat::Int64 => "i64",
                IntFormat::Default => "i32",
            },
            FieldValue::Float(format, _, _) => match format {
                FloatFormat::F32 => "f32",
                FloatFormat::F64 | FloatFormat::Default => "f64",
                FloatFormat::F128 => "f128",
            },
            FieldValue::Boolean(_, _) => "bool",
        };

        let formatted_value = handle_optional_value(value.to_string(), self.is_required());

        string_builder.push_str(&formatted_value);
        string_builder
    }

    pub fn is_required(&self) -> bool {
        match self {
            FieldValue::String(_, required, _) => *required,
            FieldValue::Int(_, required, _) => *required,
            FieldValue::Float(_, required, _) => *required,
            FieldValue::Boolean(required, _) => *required,
        }
    }

    pub fn get_description(&self) -> Option<String> {
        match self {
            FieldValue::String(_, _, description) => description.clone(),
            FieldValue::Int(_, _, description) => description.clone(),
            FieldValue::Float(_, _, description) => description.clone(),
            FieldValue::Boolean(_, description) => description.clone(),
        }
    }
}

impl Into<FieldValue> for Mapping {
    fn into(self) -> FieldValue {
        let r#type = self
            .get("type")
            .map(|v| v.as_str())
            .flatten()
            .map(|v| v.to_string())
            .expect("No type found");

        let format = self
            .get("format")
            .map(|v| v.as_str())
            .flatten()
            .map(|v| v.to_string());

        let required = self
            .get("required")
            .map(|v| v.as_bool())
            .flatten()
            .unwrap_or(false);

        let description = self
            .get("description")
            .map(|v| v.as_str())
            .flatten()
            .map(|v| v.to_string());

        match r#type.as_str() {
            "string" => FieldValue::String(
                StringFormat::from_string(format),
                required,
                description,
            ),
            "int" => FieldValue::Int(
                IntFormat::from_string(format),
                required,
                description,
            ),
            "float" => FieldValue::Float(
                FloatFormat::from_string(format),
                required,
                description,
            ),
            "boolean" => FieldValue::Boolean(required, description),
            _ => panic!("Invalid type"),
        }
    }
}


fn handle_optional_value(value: String, required: bool) -> String {
    if required {
        format!("Option<{}>", value)
    } else {
        value
    }
}