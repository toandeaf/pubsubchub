use crate::events::{Event, JsonData};
use google_cloud_gax::grpc::codegen::tokio_stream::StreamExt;
use google_cloud_pubsub::client::Client;

// TODO investigate means to pass about the same singleton publisher/sub instances.
pub async fn publish_event<Req, Res>(
    client: &Client,
    topic_id: &str,
    event_to_publish: Event<Req>,
) -> Event<Res>
where
    Req: JsonData,
    Res: JsonData + Send + 'static,
{
    // TODO dynamic inference of response type_id
    let response_type_id: u32 = 2;
    let expected_correlation_id = event_to_publish
        .correlation_id
        .clone()
        .expect("Need correlation ID");

    let subscription = client.subscription(topic_id);

    let publisher = client.topic(topic_id).new_publisher(None);
    let awaiter = publisher.publish(event_to_publish.into()).await;
    let await_result = awaiter.get().await;

    match await_result {
        Ok(_) => println!("Request sent!"),
        Err(err_status) => println!("Error status is {}", err_status),
    }

    let future = tokio::spawn(async move {
        loop {
            let mut messages = subscription
                .subscribe(None)
                .await
                .expect("Failed to pull messages");

            if let Some(message) = messages.next().await {
                let type_id = message
                    .message
                    .attributes
                    .get("type_id")
                    .map(|type_id| type_id.parse::<u32>().expect("Failed to parse"))
                    .expect("No type id available");

                let correlation_id = message
                    .message
                    .attributes
                    .get("correlation_id")
                    .expect("Failed to get correlation ID");

                if type_id == response_type_id && correlation_id == &expected_correlation_id {
                    message.ack().await.expect("Failed to acknowledge message");
                    return message.message.into();
                }
            }
        }
    });

    future.await.expect("Failed to fetch response.")
}
