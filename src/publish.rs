use crate::events::{Event, EventData, EventMeta};
use google_cloud_gax::grpc::codegen::tokio_stream::StreamExt;
use google_cloud_pubsub::client::Client;

// TODO investigate means to pass about the same singleton publisher/sub instances.
pub async fn publish_event_and_return_response<Req, Res>(
    client: &Client,
    request_topic: &str,
    response_topic: &str,
    event_to_publish: Event<Req>,
) -> Result<Event<Res>, ()>
where
    Req: EventData,
    Res: EventData + 'static,
{
    let response_type_id: u32 = Res::get_type_id();

    let expected_correlation_id = event_to_publish
        .correlation_id
        .clone()
        .expect("Need correlation ID");

    let subscription = client.subscription(response_topic);

    let publisher = client.topic(request_topic).new_publisher(None);
    let awaiter = publisher.publish(event_to_publish.into()).await.get().await;

    match awaiter {
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

                println!("got message with type id {} and correlation_id {}", type_id, correlation_id);

                if type_id == response_type_id && correlation_id == &expected_correlation_id {
                    message.ack().await.expect("Failed to acknowledge message");
                    return Ok(message.message.into());
                }
            }
        }
    });

    future.await.expect("Failed to fetch response.")
}
