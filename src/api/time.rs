use tracing::{debug, error, info};

use crate::models::time::{TimeRequest, TimeResponse};

mod response;

const GEO_LOCATION_ENDPOINT: &str = "https://api.ipgeolocation.io/timezone";

pub async fn get_local_time(
    api_key: &str,
    time_request: TimeRequest,
) -> anyhow::Result<TimeResponse> {
    // Validate request parameters
    if time_request.city.is_empty() || time_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    let location = format!("{},{}", time_request.city, time_request.country);
    info!("Fetching time data for location: {}", location);
    let url = format!(
        "{}?apiKey={}&location={}",
        GEO_LOCATION_ENDPOINT, api_key, location
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let weather_response: response::TimeResponse = response.json().await?;
        debug!("Time data fetched successfully: {:?}", weather_response);
        Ok(TimeResponse {
            city: time_request.city,
            country: time_request.country,
            local_time: weather_response.time_12,
            date: weather_response.date,
        })
    } else {
        error!("Failed to fetch time data: {}", response.status());
        Err(anyhow::anyhow!("Failed to fetch time data"))
    }
}
