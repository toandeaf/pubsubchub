use crate::events::{Event, EventCreator, JsonData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Start of docs
pub struct ConfigRequestEvent {
    event: Event<ConfigRequestContent>
}

#[derive(Serialize, Deserialize)]
pub struct ConfigRequestContent {
    pub config_id: u32,
}

impl ConfigRequestEvent {
    fn create_base_event(content: ConfigRequestContent, correlation_id: Option<String>) -> Event<ConfigRequestContent> {
        Event {
            type_id: 1,
            group_id: 1,
            correlation_id,
            content,
        }
    }
}

impl EventCreator<ConfigRequestContent> for ConfigRequestEvent {
    fn create_event(content: ConfigRequestContent) -> Event<ConfigRequestContent> {
        ConfigRequestEvent::create_base_event(content, Some(Uuid::new_v4().to_string()))
    }
    fn create_response_event(content: ConfigRequestContent, correlation_id: String) -> Event<ConfigRequestContent> {
        ConfigRequestEvent::create_base_event(content, Some(correlation_id))
    }
}

impl JsonData for ConfigRequestContent {}
/// End of docs

pub struct ConfigResponseEvent {
    event: Event<ConfigResponseContent>
}

#[derive(Serialize, Deserialize)]
pub struct ConfigResponseContent {
    pub configs: HashMap<String, String>,
}

impl ConfigResponseEvent {
    fn create_base_event(content: ConfigResponseContent, correlation_id: Option<String>) -> Event<ConfigResponseContent> {
        Event {
            type_id: 2,
            group_id: 1,
            correlation_id,
            content,
        }
    }
}

impl EventCreator<ConfigResponseContent> for ConfigResponseEvent {
    fn create_event(content: ConfigResponseContent) -> Event<ConfigResponseContent> {
        ConfigResponseEvent::create_base_event(content, Some(Uuid::new_v4().to_string()))
    }
    fn create_response_event(content: ConfigResponseContent, correlation_id: String) -> Event<ConfigResponseContent> {
        ConfigResponseEvent::create_base_event(content, Some(correlation_id))
    }
}

impl JsonData for ConfigResponseContent {}
