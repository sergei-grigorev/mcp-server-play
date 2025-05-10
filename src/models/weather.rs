use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, schemars::JsonSchema)]
pub struct WeatherRequest {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
}

#[derive(Serialize, Debug, Clone, schemars::JsonSchema)]
pub struct WeatherForecast {
    #[schemars(description = "Day of the week")]
    pub day: String,
    #[schemars(description = "Temperature")]
    pub temperature: f32,
    #[schemars(description = "Conditions")]
    pub conditions: String,
}

#[derive(Serialize, Debug, Clone, schemars::JsonSchema)]
pub struct WeatherResponse {
    #[schemars(description = "City name")]
    pub city: String,
    #[schemars(description = "Country name")]
    pub country: String,
    #[schemars(description = "Temperature")]
    pub temperature: f32,
    #[schemars(description = "Conditions")]
    pub conditions: String,
    #[schemars(description = "Humidity")]
    pub humidity: u8,
    #[schemars(description = "Wind speed")]
    pub wind_speed: u8,
    #[schemars(description = "Forecast")]
    pub forecast: Vec<WeatherForecast>,
}
