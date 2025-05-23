use tracing::{debug, error, info};

use crate::api::weather::response::WeatherResponse as WeatherApiResponse;
use crate::models::weather::{WeatherRequest, WeatherResponse};

mod response;

// Default weather API endpoint
const DEFAULT_WEATHER_ENDPOINT: &str = "https://api.weatherapi.com/v1/current.json";

/// Fetch weather data for a given location
///
/// # Arguments
/// * `api_key` - API key for the weather service
/// * `weather_request` - Request containing location and unit preference
/// * `endpoint` - Optional custom endpoint URL (useful for testing)
///
/// # Returns
/// Weather response with temperature, conditions, and humidity
pub async fn get_weather(
    api_key: &str,
    weather_request: WeatherRequest,
    endpoint: Option<&str>,
) -> anyhow::Result<WeatherResponse> {
    // Validate request parameters
    if weather_request.city.is_empty() || weather_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    info!("Fetching weather data for location: {:?}", weather_request);
    let location = format!("{},{}", weather_request.city, weather_request.country);
    let weather_endpoint = endpoint.unwrap_or(DEFAULT_WEATHER_ENDPOINT);
    let url = format!("{}?key={}&q={}", weather_endpoint, api_key, location);
    debug!("Request URL: {}", url);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    debug!("Response status: {}", response.status());

    if response.status().is_success() {
        let weather_response: WeatherApiResponse = response.json().await?;
        debug!("Weather data fetched successfully: {:?}", weather_response);
        Ok(WeatherResponse {
            city: weather_request.city,
            country: weather_request.country,
            unit: weather_request.unit.clone(),
            temperature: match weather_request.unit.as_str() {
                "F" => weather_response.current.temp_f as f32,
                _ => weather_response.current.temp_c as f32,
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
    use mockito::Server;

    #[tokio::test]
    async fn empty_location_parameters_are_not_allowed() {
        let api_key = "123";

        let weather_request_empty_city = WeatherRequest {
            city: "".to_string(),
            country: "US".to_string(),
            unit: "C".to_string(),
        };

        let result1 = get_weather(api_key, weather_request_empty_city, None).await;
        assert!(result1.is_err());

        let weather_request_empty_country = WeatherRequest {
            city: "New York".to_string(),
            country: "".to_string(),
            unit: "C".to_string(),
        };

        let result2 = get_weather(api_key, weather_request_empty_country, None).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn successful_weather_request() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("key".into(), "test_api_key".into()),
                mockito::Matcher::UrlEncoded("q".into(), "London,UK".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "location": {
                    "name": "London",
                    "region": "City of London, Greater London",
                    "country": "United Kingdom"
                },
                "current": {
                    "temp_c": 15.0,
                    "temp_f": 59.0,
                    "condition": {
                        "text": "Partly cloudy",
                        "icon": "//cdn.weatherapi.com/weather/64x64/day/116.png",
                        "code": 1003
                    },
                    "humidity": 72
                }
            }"#,
            )
            .create_async()
            .await;

        let mock_endpoint = server.url();

        let weather_request = WeatherRequest {
            city: "London".to_string(),
            country: "UK".to_string(),
            unit: "C".to_string(),
        };

        let result = get_weather("test_api_key", weather_request, Some(&mock_endpoint)).await;
        println!("Response status: {:?}", result);
        if let Err(ref e) = result {
            println!("Error: {}", e);
        }
        assert!(result.is_ok());
        let weather = result.unwrap();
        assert_eq!(weather.city, "London");
        assert_eq!(weather.country, "UK");
        assert_eq!(weather.temperature, 15.0);
        assert_eq!(weather.conditions, "Partly cloudy");
        assert_eq!(weather.humidity, 72f32);
    }

    #[tokio::test]
    async fn failed_weather_request() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("key".into(), "test_api_key".into()),
                mockito::Matcher::UrlEncoded("q".into(), "NonExistent,NoCountry".into()),
            ]))
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": {
                    "code": 1006,
                    "message": "No matching location found."
                }
            }"#,
            )
            .create_async()
            .await;

        let mock_endpoint = server.url();

        let weather_request = WeatherRequest {
            city: "NonExistent".to_string(),
            country: "NoCountry".to_string(),
            unit: "C".to_string(),
        };

        let result = get_weather("test_api_key", weather_request, Some(&mock_endpoint)).await;
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("Failed to fetch weather data"));
    }
}
