use std::fs;

use osrm_openapi_models::models::{Route, RouteResponse};

pub fn load_route_response(file_path: &str) -> RouteResponse {
    // Read and parse the JSON file into RouteResponse
    let json_str = fs::read_to_string(file_path)
        .unwrap_or_else(|error| panic!("Failed to read file: {}, {}", file_path, error));

    let route_response: RouteResponse = serde_json::from_str(&json_str)
        .unwrap_or_else(|error| panic!("Failed to parse JSON from: {}, {}", file_path, error));

    route_response
}

pub fn load_route(file_path: &str, route_index: usize) -> Route {
    let route_response = load_route_response(file_path);

    route_response
        .routes
        .unwrap_or_else(|| panic!("No routes on route response: {},", file_path))
        .get(route_index)
        .unwrap_or_else(|| panic!("No route at index: {},", route_index))
        .clone()
}
