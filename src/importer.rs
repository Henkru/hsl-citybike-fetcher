use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};
use station::Station;

pub struct Importer<'a> {
    client: HttpClient<'a>
}

impl<'a> Importer<'a> {
    pub fn new(credentials: Credentials<'a>, host: &'a str) -> Importer<'a> {
        Importer {
            client: create_client(credentials, vec![host])
        }
    }

    pub fn add(&self, stations: Vec<Station>) -> Result<(), String> {
        let mut measurements = Vec::new();

        for station in stations {
            let mut measurement = Measurement::new("station");
            measurement.add_tag("geohash", station.geohash());
            measurement.add_tag("name", station.name);
            measurement.add_tag("stationId", station.station_id);

            measurement.add_field("bikesAvailable", Value::Integer(station.bikes_available));
            measurement.add_field("spacesAvailable", Value::Integer(station.spaces_available));

            measurements.push(measurement);
        }

        self.client.write_many(&measurements, None)
            .map_err(|err| format!("{:?}", err).to_string())
    }
}
