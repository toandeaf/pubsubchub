
#[derive(Debug)]
pub enum StringFormat {
    Default,
    Date,
    DateTime,
    Email,
}

impl StringFormat {
    pub(crate) fn from_string(format_opt: Option<String>) -> Self {
        if let Some(val) = format_opt {
            return match val.as_str() {
                "date" => StringFormat::Date,
                "datetime" => StringFormat::DateTime,
                "email" => StringFormat::Email,
                _ => panic!("Invalid string format"),
            };
        };

        StringFormat::Default
    }
}