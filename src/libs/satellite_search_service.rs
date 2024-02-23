use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(rename = "totalItems")]
    total_items: i32,
    member: Vec<Satellite>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Satellite {
    #[serde(rename = "satelliteId")]
    pub id: i32,
    pub name: String,
    #[serde(rename = "line1")]
    pub line_one: String,
    #[serde(rename = "line2")]
    pub line_two: String,
}

// Builds the query URL based on the provided parameters
fn build_query_url(
    base_url: &str,
    satellite_name: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
) -> String {
    let mut url = reqwest::Url::parse(base_url).expect("Base URL is invalid");
    {
        let mut query_pairs = url.query_pairs_mut();
        if let Some(name) = satellite_name {
            query_pairs.append_pair("search", name);
        }
        if let Some(p) = page {
            query_pairs.append_pair("page", &p.to_string());
        }
        if let Some(size) = page_size {
            query_pairs.append_pair("page-size", &size.to_string());
        }
    }

    url.to_string()
}

// Performs the API call and returns the response body as a string
async fn fetch_satellites_data(url: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(&url).send().await?;
    let body = res.text().await;
    body
}

// Deserializes the JSON response into a vector of Satellites
fn parse_satellites_data(body: &str) -> (Vec<Satellite>, i32) {
    serde_json::from_str::<SearchResponse>(body)
        .map(|response| (response.member, (response.total_items + 19) / 20)) // Assuming 20 items per page
        .unwrap_or_else(|err| {
            println!("Error deserializing response: {}", err);
            (vec![], 0)
        })
}

pub fn get_satellites(
    satellite_name: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
) -> (Vec<Satellite>, i32) {
    let rt = Runtime::new().unwrap();
    let base_url = "http://tle.ivanstanojevic.me/api/tle";

    let url = build_query_url(base_url, satellite_name, page, page_size);

    rt.block_on(async {
        match fetch_satellites_data(url).await {
            Ok(body) => parse_satellites_data(&body),
            Err(err) => {
                println!("Error fetching satellite data: {}", err);
                (vec![], 0)
            }
        }
    })
}
