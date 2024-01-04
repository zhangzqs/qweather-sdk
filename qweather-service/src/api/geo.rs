pub use self::city_lookup::{CityLookUpInput, CityLookUpOutput};
use anyhow::Result;
use qweather_http_client::AsyncHttpClient;
mod city_lookup;

pub struct GeoAPI<'a, C: AsyncHttpClient> {
    client: &'a C,
}

impl<'a, C: AsyncHttpClient> GeoAPI<'a, C> {
    pub fn new(client: &'a C) -> GeoAPI<'a, C> {
        Self { client }
    }
}

impl<C: AsyncHttpClient> GeoAPI<'_, C> {
    pub async fn city_lookup(&self, input: &CityLookUpInput) -> Result<CityLookUpOutput> {
        city_lookup::city_lookup(self.client, input).await
    }
}
