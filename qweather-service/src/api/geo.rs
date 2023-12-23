use qweather_http_client::{AHttpClient, HttpRequest};

pub use self::city_lookup::{CityLookUpInput, CityLookUpOutput};
use anyhow::Result;
mod city_lookup;

pub struct GeoAPI<'a, C: AHttpClient> {
    client: &'a C,
}

impl<'a, C: AHttpClient> GeoAPI<'a, C> {
    pub fn new(client: &'a C) -> GeoAPI<'a, C> {
        Self { client }
    }
}

impl<'a, C: AHttpClient> GeoAPI<'a, C> {
    pub fn city_lookup(&self, input: &'a CityLookUpInput) -> Result<CityLookUpOutput> {
        self.client.get(HttpRequest {
            base_url: "https://geoapi.qweather.com/v2/city/lookup".to_string(),
            query: input.to_hash_map(),
        })
    }
}
