#[cfg(feature = "reqwest-http-client")]
mod reqwest;
#[cfg(feature = "reqwest-http-client")]
pub use reqwest::ReqwestHttpClient;

use anyhow::Result;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub static GEO_API_URL: &str = "https://geoapi.qweather.com";
pub static WEATHER_API_URL: &str = "https://api.qweather.com";
pub static WEATHER_DEV_API_URL: &str = "https://devapi.qweather.com";


#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub url: String,
    pub query: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct StaticHttpClientConfigurationProvider<'a> {
    pub key: Option<&'a str>,
    pub geo_base_url: Option<&'a str>,
    pub weather_base_url: Option<&'a str>,
}

impl HttpClientConfigurationProvider for StaticHttpClientConfigurationProvider<'_> {
    fn key(&self) -> Option<&str> {
        self.key
    }

    fn geo_base_url(&self) -> &str {
        self.geo_base_url.unwrap_or(GEO_API_URL)
    }

    fn weather_base_url(&self) -> &str {
        self.weather_base_url.unwrap_or(WEATHER_API_URL)
    }
}

pub trait HttpClientConfigurationProvider {
    fn key(&self) -> Option<&str>;
    fn geo_base_url(&self) -> &str;
    fn weather_base_url(&self) -> &str;
}

pub trait AHttpClient {
    type ConfigProvider: HttpClientConfigurationProvider;
    fn config(&self) -> &Self::ConfigProvider;
    fn get<T: DeserializeOwned>(&self, req: HttpRequest) -> Result<T>;
}
