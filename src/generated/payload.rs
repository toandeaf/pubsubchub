use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct GetConfig {
    /// The ID of the configuration to retrieve
    pub config_id: Option<i32>,
    /// The username of the user requesting the configuration
    pub username: String,
    /// The email of the user requesting the configuration
    pub email: String,
}

#[derive(Deserialize, Serialize)]
struct GetConfigRequest {
    /// The ID of the configuration to retrieve
    pub config_id: Option<i32>,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
struct GetConfigResponse {
    /// The ID of the configuration to retrieve
    pub data: String,
}

