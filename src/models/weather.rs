use rmcp::schemars;
use serde::{Deserialize, Serialize};

fn default_temperature_unit() -> String {
    "C".to_string()
}

#[derive(Deserialize, Debug, schemars::JsonSchema)]
pub struct WeatherRequest {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Temperature unit (C or F)", default = "default_temperature_unit")]
    pub unit: String,
}

#[derive(Serialize, Debug, Clone, schemars::JsonSchema)]
pub struct WeatherResponse {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Temperature unit (C or F)")]
    pub unit: String,
    #[schemars(description = "Temperature")]
    pub temperature: f32,
    #[schemars(description = "Conditions")]
    pub conditions: String,
    #[schemars(description = "Humidity")]
    pub humidity: f32,
}
