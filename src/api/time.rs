use tracing::{debug, error, info};

use crate::models::time::{TimeRequest, TimeResponse};

mod response;

const DEFAULT_GEO_LOCATION_ENDPOINT: &str = "https://api.ipgeolocation.io/timezone";

/// Fetch local time data for a given location
///
/// # Arguments
/// * `api_key` - API key for the geolocation service
/// * `time_request` - Request containing location information
/// * `endpoint` - Optional custom endpoint URL (useful for testing)
///
/// # Returns
/// Time response with local time and date information
pub async fn get_local_time(
    api_key: &str,
    time_request: TimeRequest,
    endpoint: Option<&str>,
) -> anyhow::Result<TimeResponse> {
    // Validate request parameters
    if time_request.city.is_empty() || time_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    let location = format!("{},{}", time_request.city, time_request.country);
    info!("Fetching time data for location: {}", location);
    let time_endpoint = endpoint.unwrap_or(DEFAULT_GEO_LOCATION_ENDPOINT);
    let url = format!("{}?apiKey={}&location={}", time_endpoint, api_key, location);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let time_response: response::TimeResponse = response.json().await?;
        debug!("Time data fetched successfully: {:?}", time_response);
        Ok(TimeResponse {
            city: time_request.city,
            country: time_request.country,
            local_time: time_response.time_12,
            date: time_response.date,
        })
    } else {
        error!("Failed to fetch time data: {}", response.status());
        Err(anyhow::anyhow!("Failed to fetch time data"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn empty_city_or_country_not_allowed() {
        let api_key = "test_api_key";
        let mut request = TimeRequest {
            city: "New York".to_string(),
            country: "United States".to_string(),
        };

        request.city = "".to_string();
        let result = get_local_time(api_key, request.clone(), None).await;
        assert!(result.is_err());

        request.city = "New York".to_string();
        request.country = "".to_string();
        let result = get_local_time(api_key, request, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn successful_time_request() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("apiKey".into(), "test_api_key".into()),
                mockito::Matcher::UrlEncoded("location".into(), "London,UK".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "time_12": "08:30 PM",
                    "date": "2023-05-14"
                }"#,
            )
            .create_async()
            .await;

        let request = TimeRequest {
            city: "London".to_string(),
            country: "UK".to_string(),
        };

        let result = get_local_time("test_api_key", request, Some(&server.url()))
            .await
            .unwrap();

        assert_eq!(result.city, "London");
        assert_eq!(result.country, "UK");
        assert_eq!(result.local_time, "08:30 PM");
        assert_eq!(result.date, "2023-05-14");
    }

    #[tokio::test]
    async fn failed_time_request() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("apiKey".into(), "test_api_key".into()),
                mockito::Matcher::UrlEncoded("location".into(), "Invalid,Location".into()),
            ]))
            .with_status(404)
            .create_async()
            .await;

        let request = TimeRequest {
            city: "Invalid".to_string(),
            country: "Location".to_string(),
        };

        let result = get_local_time("test_api_key", request, Some(&server.url())).await;

        assert!(result.is_err());
    }
}
