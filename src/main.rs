use crate::config::{
    ConfigRequestContent, ConfigRequestEvent, ConfigResponseContent,
};
use crate::events::{Event, EventCreator, JsonData};
use crate::publish::publish_event_and_return_response;
use crate::setup::setup;
use crate::subscribe::consume_event;
use juniper::futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        // TODO add ::new as the only public accessor constructor
        let content = ConfigRequestContent { config_id: 1 };
        let event = ConfigRequestEvent::create_event(content);

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

    Event {
        type_id: 2,
        group_id: 1,
        correlation_id: req_event.correlation_id,
        content: ConfigResponseContent { configs },
    }
}
