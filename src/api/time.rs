use crate::models::time::{TimeRequest, TimeResponse};

pub async fn get_local_time(time_request: TimeRequest) -> anyhow::Result<TimeResponse> {
    // Validate request parameters
    if time_request.city.is_empty() || time_request.country.is_empty() {
        return Err(anyhow::anyhow!("City and country cannot be empty"));
    }

    // Return mock time data
    Ok(TimeResponse {
        city: time_request.city,
        country: time_request.country,
        local_time: "15:45:30".to_string(),
        timezone: "UTC+2".to_string(),
        date: "2025-05-08".to_string(),
    })
}
