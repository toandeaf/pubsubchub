#[derive(Debug)]
pub enum IntFormat {
    Default,
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
}

impl IntFormat {
    pub(crate) fn from_string(format_opt: Option<String>) -> Self {
        if let Some(format) = format_opt {
            return match format.as_str() {
                "uint8" => IntFormat::Uint8,
                "int8" => IntFormat::Int8,
                "uint16" => IntFormat::Uint16,
                "int16" => IntFormat::Int16,
                "uint32" => IntFormat::Uint32,
                "int32" => IntFormat::Int32,
                "uint64" => IntFormat::Uint64,
                "int64" => IntFormat::Int64,
                _ => panic!("Invalid int format"),
            };
        };

        IntFormat::Default
    }
}
