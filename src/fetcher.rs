extern crate reqwest;

use std::collections::HashMap;
use station::Station;

const API_URL: &str = "https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql";

lazy_static! {
    static ref CLIENT: reqwest::Client = {
        reqwest::Client::new()
    };
}

lazy_static! {
    static ref QUERY: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("query", "{ bikeRentalStations { stationId name bikesAvailable spacesAvailable lat lon } }");
        m
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BikeRentailStations {
    bike_rental_stations: Vec<Station>
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StationResponse {
    data: BikeRentailStations
}

pub fn fetch() -> Result<Vec<Station>, String> {
    let query: &HashMap<&'static str, &'static str> = &QUERY;
    let response = CLIENT.post(API_URL)
        .json(query)
        .send();

    response.and_then(|mut response| response.json::<StationResponse>())
        .map(|json| json.data.bike_rental_stations)
        .map_err(|err| format!("{:?}", err).to_string())
}
