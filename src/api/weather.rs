use crate::models::weather::{WeatherForecast, WeatherRequest, WeatherResponse};

pub async fn get_weather(weather_request: WeatherRequest) -> anyhow::Result<WeatherResponse> {
    // Validate request parameters
    if weather_request.city.is_empty() || weather_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    // Return mock weather data
    Ok(WeatherResponse {
        city: weather_request.city,
        country: weather_request.country,
        temperature: 22.5,
        conditions: "Sunny".to_string(),
        humidity: 45,
        wind_speed: 15,
        forecast: [
            WeatherForecast {
                day: "Today".to_string(),
                temperature: 22.5,
                conditions: "Sunny".to_string(),
            },
            WeatherForecast {
                day: "Tomorrow".to_string(),
                temperature: 20.0,
                conditions: "Partly Cloudy".to_string(),
            },
            WeatherForecast {
                day: "Day after tomorrow".to_string(),
                temperature: 18.0,
                conditions: "Light Rain".to_string(),
            },
        ]
        .to_vec(),
    })
}
