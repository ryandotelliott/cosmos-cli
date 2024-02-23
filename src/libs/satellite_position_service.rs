use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    timestamp: i64,
    satlatitude: f64,
    satlongitude: f64,
    sataltitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionsResponse {
    positions: Vec<Position>,
}

// Builds the query URL based on the provided parameters
fn build_query_url(base_url: &str, satellite_id: i32) -> String {
    // We don't care about the observer's location for now; since that's used for values that we don't display
    const OBSERVER_LAT: f64 = 40.7128;
    const OBSERVER_LON: f64 = 74.0060;
    const OBSERVER_ALT: f64 = 0.0;
    const SECONDS: i32 = 1;

    // Format of the request is weird, every item is positional
    // /positions/{id}/{observer_lat}/{observer_lng}/{observer_alt}/{seconds}?apiKey=api-key
    let mut url = reqwest::Url::parse(&format!(
        "{}/{}/{}/{}/{}/{}",
        base_url, satellite_id, OBSERVER_LAT, OBSERVER_LON, OBSERVER_ALT, SECONDS
    ))
    .expect("Base URL is invalid");
    {
        let api_key = env::var("N2YO_API_KEY").expect("N2YO_API_KEY must be set");
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair("apiKey", &api_key);
    }

    url.to_string()
}

/**
Fetches the list of satellites from the API

# Arguments
* `satellite_name` - The name of the satellite to search for
# Returns
* A tuple containing the latitude and longitude of the satellite
*/
pub fn get_satellite_position(satellite_id: i32) -> (f64, f64) {
    let rt = Runtime::new().unwrap();
    let base_url = "https://api.n2yo.com/rest/v1/satellite/positions";

    let url = build_query_url(base_url, satellite_id);

    rt.block_on(async {
        let client = Client::new();
        let response = client.get(&url).send().await.unwrap();
        let positions: PositionsResponse = response.json().await.unwrap_or_else(|err| {
            eprintln!("Error deserializing satellite position response: {}", err);
            PositionsResponse { positions: vec![] }
        });

        let position = positions.positions.first().unwrap();
        (position.satlatitude, position.satlongitude)
    })
}
