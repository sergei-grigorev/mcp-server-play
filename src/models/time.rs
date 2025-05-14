use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, schemars::JsonSchema)]
pub struct TimeRequest {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
}

#[derive(Serialize, Debug, schemars::JsonSchema)]
pub struct TimeResponse {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Local time")]
    pub local_time: String,
    #[schemars(description = "Date")]
    pub date: String,
}
