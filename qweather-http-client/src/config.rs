use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct StaticHttpClientConfigurationProvider<'a> {
    pub key: Option<&'a str>,
    pub geo_base_url: Option<&'a str>,
    pub weather_base_url: Option<&'a str>,
}

pub static GEO_API_URL: &str = "https://geoapi.qweather.com";
pub static WEATHER_API_URL: &str = "https://api.qweather.com";
pub static WEATHER_DEV_API_URL: &str = "https://devapi.qweather.com";

#[async_trait(?Send)]
impl AsyncHttpClientConfigurationProvider for StaticHttpClientConfigurationProvider<'_> {
    async fn key(&self) -> Option<&str> {
        self.key
    }

    async fn geo_base_url(&self) -> &str {
        self.geo_base_url.unwrap_or(GEO_API_URL)
    }

    async fn weather_base_url(&self) -> &str {
        self.weather_base_url.unwrap_or(WEATHER_API_URL)
    }
}

#[async_trait(?Send)]
pub trait AsyncHttpClientConfigurationProvider {
    async fn key(&self) -> Option<&str>;
    async fn geo_base_url(&self) -> &str;
    async fn weather_base_url(&self) -> &str;
}
