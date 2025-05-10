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

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/mcp-server.git
   cd mcp-server
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Running the Server

Run the server using Cargo:

```bash
cargo run
```

## Project Structure

- `src/main.rs`: Entry point and server initialization
- `src/api/mod.rs`: Main server implementation
- `src/api/weather.rs`: Weather data retrieval logic
- `src/api/time.rs`: Local time retrieval logic
- `src/models/`: Data models for requests

## Tools and Endpoints

### Weather Information

- **Endpoint**: `get_weather`
- **Parameters**:
  - `city`: Name of the city
  - `country`: Country code

### Local Time

- **Endpoint**: `get_local_time`
- **Parameters**:
  - `city`: Name of the city
  - `country`: Country code

## License

MIT
