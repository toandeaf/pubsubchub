use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::string::ToString;

const FAILURE_MESSAGE: &str = "{\"message\": \"failed to serialize data\"}";

pub trait JsonData: Serialize + DeserializeOwned {}

pub trait EventCreator<T> where T: JsonData {
    fn create_event(content: T) -> Event<T>;
    fn create_response_event(content: T, correlation_id: String) -> Event<T>;
}

pub struct Event<T>
where
    T: JsonData,
{
    pub type_id: u32,
    pub group_id: u32,
    pub correlation_id: Option<String>,
    pub content: T,
}

impl<T> Into<Event<T>> for PubsubMessage
where
    T: JsonData,
{
    fn into(self) -> Event<T> {
        let type_id = self
            .attributes
            .get("type_id")
            .map(|val| val.parse::<u32>().unwrap_or(0))
            .unwrap_or(0);

        let group_id = self
            .attributes
            .get("group_id")
            .map(|val| val.parse::<u32>().unwrap_or(0))
            .unwrap_or(0);

        let correlation_id = self.attributes.get("correlation_id").cloned();

        let content = serde_json::from_slice(self.data.as_slice()).unwrap();

        Event {
            type_id,
            group_id,
            correlation_id,
            content,
        }
    }
}

impl<T> Into<PubsubMessage> for Event<T>
where
    T: JsonData,
{
    fn into(self) -> PubsubMessage {
        let data = serde_json::to_string(&self.content)
            .unwrap_or(FAILURE_MESSAGE.to_string())
            .into_bytes();

        let mut attributes = HashMap::with_capacity(2);

        attributes.insert("type_id".into(), self.type_id.to_string());
        attributes.insert("group_id".into(), self.group_id.to_string());

        if let Some(uuid) = self.correlation_id {
            attributes.insert("correlation_id".to_string(), uuid.to_string());
        }

        PubsubMessage {
            attributes,
            data,
            ..Default::default()
        }
    }
}
