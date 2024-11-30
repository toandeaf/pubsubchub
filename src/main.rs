use crate::events::{ConfigRequestContent, ConfigResponseContent, Event, JsonData};
use crate::publish::publish_event;
use crate::setup::setup;
use crate::subscribe::consume_event;
use juniper::futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
        let event: Event<ConfigRequestContent> = Event {
            type_id: 1,
            group_id: 1,
            correlation_id: Some(Uuid::new_v4().to_string()),
            content: ConfigRequestContent { config_id: 1 },
        };

        let returned_event: Event<ConfigResponseContent> =
            publish_event(&client, "config", event).await;

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
