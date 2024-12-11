#[derive(Debug)]
pub enum FloatFormat {
    Default,
    F32,
    F64,
    F128,
}


impl FloatFormat {
    pub(crate) fn from_string(format_opt: Option<String>) -> Self {
        if let Some(format) = format_opt {
            return match format.as_str() {
                "f32" => FloatFormat::F32,
                "f64" => FloatFormat::F64,
                "f128" => FloatFormat::F128,
                _ => panic!("Invalid float format"),
            };
        };

        FloatFormat::Default
    }
}