use crate::events::{EventCreator, EventData};
use crate::types::{extract_payload_data, Converter, Payload, RustConverter};
use juniper::futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;

mod config;
mod events;
mod generated;
mod publish;
mod setup;
mod subscribe;
mod types;

#[tokio::main]
async fn main() {
    // TODO traverse the schema directory and generate the necessary files
    let data_from_file =
        File::open("./schema/option_two/config/payload.yml").expect("Failed to open file");

    let parsed_vec: Vec<Payload> = extract_payload_data(data_from_file);

    RustConverter::convert_payloads(parsed_vec);
}
