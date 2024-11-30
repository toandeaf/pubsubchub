use google_cloud_gax::conn::{ConnectionOptions, Environment};
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::SubscriptionConfig;
use std::time::Duration;

pub async fn setup(topics: Vec<&str>, subscriptions: Vec<&str>) -> Client {
    let environment = Environment::Emulator("localhost:8085".to_string());

    let client = Client::new(ClientConfig {
        environment,
        endpoint: "http://localhost:8085".to_string(),
        project_id: Some("pubsubchub".to_string()),
        connection_option: ConnectionOptions {
            connect_timeout: Some(Duration::from_secs(10)),
            timeout: Some(Duration::from_secs(10)),
        },
        ..Default::default()
    })
    .await
    .unwrap();

    create_topics_if_not_exists(&client, &topics).await;

    for topic in topics {
        create_subscriptions_if_not_exists(&client, topic, &subscriptions).await;
    }

    client
}

async fn create_topics_if_not_exists(client: &Client, topics: &Vec<&str>) {
    let existing_topics = client.get_topics(None).await.expect("Failed to get topics");

    for topic in topics {
        if !existing_topics.iter().any(|top| top.contains(topic)) {
            client
                .create_topic(topic, None, None)
                .await
                .expect("Failed to create topic");
        }
    }
}

async fn create_subscriptions_if_not_exists(
    client: &Client,
    topic_id: &str,
    subscriptions: &Vec<&str>,
) {
    let existing_subscriptions = client
        .get_subscriptions(None)
        .await
        .expect("Failed to get topics");

    for subscription in subscriptions {
        if !existing_subscriptions
            .iter()
            .any(|sub| sub.id().contains(subscription))
        {
            client
                .create_subscription(subscription, topic_id, SubscriptionConfig::default(), None)
                .await
                .expect("Failed to create subscription");
        }
    }
}
