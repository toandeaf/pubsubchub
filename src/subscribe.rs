use crate::events::{Event, JsonData};
use google_cloud_gax::grpc::codegen::tokio_stream::StreamExt;
use google_cloud_pubsub::client::Client;

pub async fn consume_event<Req, Res, Function>(client: &Client, topic_id: &str, function: Function)
where
    Req: JsonData,
    Res: JsonData,
    Function: Fn(Event<Req>) -> Event<Res>,
{
    // TODO dynamic inference of request type_id
    let expected_type_id: u32 = 1;

    let subscription = client.subscription(topic_id);
    let publisher = client.topic(topic_id).new_publisher(None);

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

            if type_id == expected_type_id {
                message.ack().await.expect("Failed to acknowledge message");

                let consumed_event: Event<Req> = message.message.into();
                let publish_event: Event<Res> = function(consumed_event);

                let res = publisher.publish(publish_event.into()).await;
                let response = res.get().await;

                match response {
                    Ok(_) => println!("Message sent!"),
                    _ => println!("There was an error"),
                }
            }
        }
    }
}
