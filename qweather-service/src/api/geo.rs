use qweather_http_client::AHttpClient;

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

impl<C: AHttpClient> GeoAPI<'_, C> {
    pub fn city_lookup(&self, input: &CityLookUpInput) -> Result<CityLookUpOutput> {
        city_lookup::city_lookup(self.client, input)
    }
}
