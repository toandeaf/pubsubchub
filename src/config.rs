use crate::events::{Event, EventCreator, EventData, EventMeta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Start of docs
#[derive(Serialize, Deserialize)]
pub struct ConfigRequestContent {
    pub config_id: u32,
}

impl EventMeta for ConfigRequestContent {
    fn get_group_id() -> u32 {
        1
    }
    fn get_type_id() -> u32 {
        1
    }
}

impl EventData for ConfigRequestContent {}
/// End of docs

/// Start of docs
#[derive(Serialize, Deserialize)]
pub struct ConfigResponseContent {
    pub configs: HashMap<String, String>,
}

impl EventMeta for ConfigResponseContent {
    fn get_group_id() -> u32 {
        1
    }
    fn get_type_id() -> u32 {
        2
    }
}

impl EventData for ConfigResponseContent {}