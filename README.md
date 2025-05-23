# MCP Server: Weather and Time Information Service

## Project Overview

This Model Context Protocol (MCP) server is a learning project that leverages the RMCP (Rust Model Context Protocol) library to demonstrate:

- Async Rust programming with tokio
- Implementing a flexible microservice architecture
- Using the RMCP SDK for building extensible tools

The server provides two primary tools:

1. **Weather Information**: Retrieve weather data for a specific city and country
2. **Local Time**: Get the current local time for a given city and country

## About RMCP

RMCP is a modern Rust SDK for the Model Context Protocol, offering:

- Clean and efficient async runtime with tokio
- Flexible and extensible tool implementation
- Improved data type handling compared to official SDKs
- Support for various transport mechanisms
  - Server-Sent Events (SSE) for real-time communication
  - Lightweight and efficient one-way server-to-client data streaming
  - Built-in reconnection support and event tracking

## Purpose

The primary goal of this project is to:

- Learn Rust programming
- Explore RMCP library capabilities

## Features

- Weather data retrieval
- Local time information
- Stdio-based communication
- Structured logging with tracing

## Prerequisites

- Rust (latest stable version)
- Cargo package manager
- [just](https://github.com/casey/just) command runner
  - Install with Homebrew: `brew install just`
  - Install with Cargo: `cargo install just`
- API keys for:
  - [WeatherAPI](https://www.weatherapi.com/) for weather data
  - [IP Geolocation API](https://ipgeolocation.io/) for time data

## Configuration

The server can be configured using the following environment variables:

- `BIND_ADDRESS`: The IP address to bind the server to (default: `0.0.0.0`)
- `PORT`: The port number to listen on (default: `8000`)

Example usage:
```bash
# Bind to localhost on port 3000
BIND_ADDRESS=127.0.0.1 PORT=3000 cargo run

# Use default bind address but change port
PORT=8080 cargo run
```

## Environment Setup

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file and replace the placeholder values with your actual API keys:
   - `WEATHER_API_KEY`: Get it from [WeatherAPI](https://www.weatherapi.com/)
   - `IP_GEOLOCATION_API_KEY`: Get it from [IP Geolocation API](https://ipgeolocation.io/)

3. Optionally, you can also configure:
   - `BIND_ADDRESS`: The IP address to bind the server to (default: `0.0.0.0`)
   - `PORT`: The port number to listen on (default: `8000`)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/sergei-grigorev/mcp-server-play.git
   cd mcp-server-play
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Development Commands

The project uses [just](https://github.com/casey/just) as a command runner. Available commands:

```bash
just                # List available commands
just build          # Build release artifact
just check          # Run checks (clippy, fmt, etc.)
just clean          # Clean build artifacts
just format         # Format code using rustfmt
just serve          # Run the server in development mode
just test           # Run tests
```

## Running the Server

Run the server using just:

```bash
just serve
```

## Project Structure

- `src/main.rs`: Entry point and server initialization
- `src/api/`
  - `mod.rs`: Main server implementation
  - `weather.rs`: Weather data retrieval logic
  - `weather/response.rs`: Weather API response models
  - `time.rs`: Time data retrieval logic
  - `time/response.rs`: Time API response models
- `src/models/`
  - `mod.rs`: Models module definitions
  - `weather.rs`: Weather request/response models
  - `time.rs`: Time request/response models

## Tools and Endpoints

### Weather Information

- **Endpoint**: `get_weather`
- **Parameters**:
  - `city`: Name of the city
  - `country`: Country code
  - `unit`: Temperature unit (optional)
    - Values: `C` (Celsius) or `F` (Fahrenheit)
    - Default: `C`

### Local Time

- **Endpoint**: `get_local_time`
- **Parameters**:
  - `city`: Name of the city
  - `country`: Country code

## License

MIT
