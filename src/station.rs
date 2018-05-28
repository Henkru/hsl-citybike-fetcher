use geohash::Coordinate;
use geohash::encode;
use cached::UnboundCache;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub station_id: String,
    pub name: String,
    pub bikes_available: i64,
    pub spaces_available: i64,
    pub lat: f64,
    pub lon: f64
}

impl Station {
    pub fn geohash(&self) -> String {
        geohash(self.station_id.clone(), self.lat, self.lon)
    }
}

cached_key! {
    GEO: UnboundCache<String, String> = UnboundCache::new();
    Key = { station_id };
    fn geohash(station_id: String, lat: f64, lon: f64) -> String = {
       let coordinate = Coordinate{x: lon, y: lat};
       encode(coordinate, 12)
    }
}
