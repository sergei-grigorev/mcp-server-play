use tracing::{debug, error, info};

use crate::api::weather::response::WeatherResponse as WeatherApiResponse;
use crate::models::weather::{TemperatureUnit, WeatherRequest, WeatherResponse};

mod response;

const WEATHER_ENDPOINT: &str = "https://api.weatherapi.com/v1/current.json";

pub async fn get_weather(
    api_key: &str,
    weather_request: WeatherRequest,
) -> anyhow::Result<WeatherResponse> {
    // Validate request parameters
    if weather_request.city.is_empty() || weather_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    info!("Fetching weather data for location: {:?}", weather_request);
    let location = format!("{},{}", weather_request.city, weather_request.country);
    let url = format!("{}?key={}&q={}", WEATHER_ENDPOINT, api_key, location);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let weather_response: WeatherApiResponse = response.json().await?;
        debug!("Weather data fetched successfully: {:?}", weather_response);
        Ok(WeatherResponse {
            city: weather_request.city,
            country: weather_request.country,
            unit: weather_request.unit.clone(),
            temperature: match weather_request.unit {
                TemperatureUnit::C => weather_response.current.temp_c as f32,
                TemperatureUnit::F => weather_response.current.temp_f as f32,
            },
            conditions: weather_response.current.condition.text,
            humidity: weather_response.current.humidity,
        })
    } else {
        error!("Failed to fetch weather data: {}", response.status());
        Err(anyhow::anyhow!("Failed to fetch weather data"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn empty_location_parameters_are_not_allowed() {
        let api_key = "123";

        let weather_request_empty_city = WeatherRequest {
            city: "".to_string(),
            country: "US".to_string(),
            unit: TemperatureUnit::C,
        };

        let result1 = get_weather(api_key, weather_request_empty_city).await;
        assert!(result1.is_err());

        let weather_request_empty_country = WeatherRequest {
            city: "New York".to_string(),
            country: "".to_string(),
            unit: TemperatureUnit::C,
        };

        let result2 = get_weather(api_key, weather_request_empty_country).await;
        assert!(result2.is_err());
    }
}
