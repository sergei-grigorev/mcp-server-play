use std::env;

use rmcp::{model::*, service::RequestContext, tool, Error as McpError, RoleServer, ServerHandler};

use crate::models::{time::TimeRequest, weather::WeatherRequest};

pub mod time;
pub mod weather;

#[derive(Clone, Debug)]
pub struct MyServer;

#[tool(tool_box)]
impl MyServer {
    pub fn new() -> Self {
        Self
    }

    #[tool(description = "Get weather data")]
    pub async fn get_weather(
        &self,
        #[tool(aggr)]
        #[schemars(description = "Request")]
        WeatherRequest {
            city,
            country,
            unit,
        }: WeatherRequest,
    ) -> Result<CallToolResult, McpError> {
        let weather_api_key = match env::var("WEATHER_API_KEY") {
            Ok(k) => k,
            Err(_) => {
                return Err(McpError::internal_error(
                    "WEATHER_API_KEY environment variable not set",
                    None,
                ))
            }
        };

        match weather::get_weather(
            &weather_api_key,
            WeatherRequest {
                city,
                country,
                unit,
            },
            None,
        )
        .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get local time")]
    pub async fn get_local_time(
        &self,
        #[tool(aggr)]
        #[schemars(description = "Request")]
        TimeRequest { city, country }: TimeRequest,
    ) -> Result<CallToolResult, McpError> {
        let geo_location_api_key = match env::var("IP_GEOLOCATION_API_KEY") {
            Ok(k) => k,
            Err(_) => {
                return Err(McpError::internal_error(
                    "IP_GEOLOCATION_API_KEY environment variable not set",
                    None,
                ))
            }
        };

        match time::get_local_time(&geo_location_api_key, TimeRequest { city, country }, None).await
        {
            Ok(response) => Ok(CallToolResult::success(vec![Content::json(response)?])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                // .enable_prompts()
                // .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides a time and weather tools.".to_string()),
        }
    }

    async fn initialize(
        &self,
        request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        tracing::info!("Client capabilities: {:#?}", request.capabilities);
        tracing::info!("Client info: {:#?}", request.client_info);
        tracing::info!("Protocol version: {:#?}", request.protocol_version);
        tracing::info!("Server info: {:#?}", self.get_info());
        tracing::info!("RequestId: {:#?}", context.id);
        Ok(self.get_info())
    }
}
