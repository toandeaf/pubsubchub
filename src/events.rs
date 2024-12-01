use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::string::ToString;
use uuid::Uuid;

const FAILURE_MESSAGE: &str = "{\"message\": \"failed to serialize data\"}";

pub trait EventData: Serialize + DeserializeOwned + Send + EventMeta {}

pub trait EventMeta {
    fn get_group_id() -> u32;
    fn get_type_id() -> u32;
}

pub trait EventCreator<T>
where
    T: EventData,
{
    fn create_event(content: T, correlation_id: Option<String>) -> Event<T>;
}

pub struct Event<T>
where
    T: EventData,
{
    pub correlation_id: Option<String>,
    pub content: T,
    pub metadata: HashMap<String, String>,
}

impl<T: EventData> EventCreator<T> for Event<T> {
    fn create_event(content: T, correlation_id: Option<String>) -> Event<T> {
        Event {
            content,
            correlation_id: correlation_id.or(Some(Uuid::new_v4().to_string())),
            metadata: HashMap::new(),
        }
    }
}

impl<T> Into<Event<T>> for PubsubMessage
where
    T: EventData,
{
    fn into(self) -> Event<T> {
        let correlation_id = self.attributes.get("correlation_id").cloned();

        let content = serde_json::from_slice(self.data.as_slice()).unwrap();

        Event {
            correlation_id,
            content,
            metadata: HashMap::new(),
        }
    }
}

impl<T> Into<PubsubMessage> for Event<T>
where
    T: EventData,
{
    fn into(self) -> PubsubMessage {
        let data = serde_json::to_string(&self.content)
            .unwrap_or(FAILURE_MESSAGE.to_string())
            .into_bytes();

        let mut attributes: HashMap<String, String> = vec![
            ("type_id".to_string(), T::get_type_id().to_string()),
            ("group_id".to_string(), T::get_group_id().to_string()),
        ]
        .into_iter()
        .collect();

        if let Some(uuid) = self.correlation_id {
            attributes.insert("correlation_id".into(), uuid.into());
        }

        PubsubMessage {
            attributes,
            data,
            ..Default::default()
        }
    }
}
