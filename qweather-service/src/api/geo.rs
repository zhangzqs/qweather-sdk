pub use self::city_lookup::{CityLookUpInput, CityLookUpOutput};
use qweather_http_client::AsyncHttpClient;
mod city_lookup;

pub struct GeoApi<'a, C: AsyncHttpClient> {
    client: &'a C,
}

impl<'a, C: AsyncHttpClient> GeoApi<'a, C> {
    pub fn new(client: &'a C) -> GeoApi<'a, C> {
        Self { client }
    }
}

impl<C: AsyncHttpClient> GeoApi<'_, C> {
    pub async fn city_lookup(&self, input: &CityLookUpInput) -> Result<CityLookUpOutput, C::Error> {
        city_lookup::city_lookup(self.client, input).await
    }
}
