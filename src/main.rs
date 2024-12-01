use crate::config::{ConfigRequestContent, ConfigResponseContent};
use crate::events::{Event, EventCreator, EventData};
use crate::publish::publish_event_and_return_response;
use crate::setup::setup;
use crate::subscribe::consume_event;
use google_cloud_pubsub::client::Client;
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
    // TODO fix the subscriptions and publishers.
    let client = setup(
        vec!["config-request", "config-response"],
        vec!["config-request", "config-response"],
    )
    .await;


    #[cfg(feature = "publish")]
    {
        let event = Event::create_event(ConfigRequestContent { config_id: 1 }, None);

        let returned_event: Result<Event<ConfigResponseContent>, ()> =
            publish_event_and_return_response(&client, "config-request", "config-response", event)
                .await;

        if let Ok(event) = returned_event {
            print!(
                "Returned event is {}",
                serde_json::to_string(&event.content).unwrap()
            );
        } else {
            println!("Couldn't fetch the response!");
        }
    }

    #[cfg(feature = "consume")]
    consume_event(
        &client,
        "config-request",
        "config-response",
        handle_config_request,
    )
    .await;
}

async fn purge_events(client: &Client, topic_id: &str) {
    client
        .topic(topic_id)
        .delete(None)
        .await
        .expect("Failed to purge");
}

fn handle_config_request(req_event: Event<ConfigRequestContent>) -> Event<ConfigResponseContent> {
    let mut configs = HashMap::new();
    configs.insert("config".to_string(), "value".to_string());

    Event::create_event(ConfigResponseContent { configs }, req_event.correlation_id)
}
