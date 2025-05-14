use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, schemars::JsonSchema)]
pub enum TemperatureUnit {
    C,
    F,
}

fn default_temperature_unit() -> TemperatureUnit {
    TemperatureUnit::C
}

#[derive(Deserialize, Debug, schemars::JsonSchema)]
pub struct WeatherRequest {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Temperature unit", default = "default_temperature_unit")]
    pub unit: TemperatureUnit,
}

#[derive(Serialize, Debug, Clone, schemars::JsonSchema)]
pub struct WeatherResponse {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Temperature unit")]
    pub unit: TemperatureUnit,
    #[schemars(description = "Temperature")]
    pub temperature: f32,
    #[schemars(description = "Conditions")]
    pub conditions: String,
    #[schemars(description = "Humidity")]
    pub humidity: f32,
}
