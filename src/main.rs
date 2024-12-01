use crate::events::{Event, EventCreator, EventData};
use crate::publish::publish_event_and_return_response;
use crate::setup::setup;
use crate::subscribe::consume_event;
use juniper::futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::config::{ConfigRequestContent, ConfigResponseContent};

mod config;
mod events;
mod publish;
mod setup;
mod subscribe;

#[tokio::main]
async fn main() {
    let client = setup(vec!["config"], vec!["config"]).await;

    #[cfg(feature = "publish")]
    {
        let event = Event::create_event(ConfigRequestContent { config_id: 1 }, None);

        let returned_event: Event<ConfigResponseContent> =
            publish_event_and_return_response(&client, "config", event).await;

        print!(
            "Returned event is {}",
            serde_json::to_string(&returned_event.content).unwrap()
        );
    }

    #[cfg(feature = "consume")]
    consume_event(&client, "config", handle_config_request).await;
}

fn handle_config_request(req_event: Event<ConfigRequestContent>) -> Event<ConfigResponseContent> {
    let mut configs = HashMap::new();
    configs.insert("config".to_string(), "value".to_string());

    Event::create_event(ConfigResponseContent { configs }, req_event.correlation_id)
}
