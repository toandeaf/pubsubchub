use crate::events::{Event, EventData};
use google_cloud_gax::grpc::codegen::tokio_stream::StreamExt;
use google_cloud_pubsub::client::Client;

pub async fn consume_event<Req, Res, Function>(
    client: &Client,
    request_topic: &str,
    response_topic: &str,
    function: Function,
) where
    Req: EventData,
    Res: EventData,
    Function: Fn(Event<Req>) -> Event<Res>,
{
    let expected_type_id: u32 = Req::get_type_id();

    let mut subscription = client.subscription(request_topic);
    let publisher = client.topic(response_topic).new_publisher(None);

    loop {
        let mut messages = subscription.subscribe(None).await.expect("Failed to pull");

        if let Some(message) = messages.next().await {
            let type_id = message
                .message
                .attributes
                .get("type_id")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            if type_id != expected_type_id {
                message.nack().await.expect("Failed to nack");
                continue;
            }

            message.ack().await.expect("Failed to acknowledge message");

            let consumed_event: Event<Req> = message.message.into();
            let publish_event: Event<Res> = function(consumed_event);

            let res = publisher.publish(publish_event.into()).await;
            let response = res.get().await;

            match response {
                Ok(_) => println!("Response sent!"),
                _ => println!("There was an error"),
            }
        }
    }
}
