use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

mod config;
pub use config::{
    AsyncHttpClientConfigurationProvider, StaticHttpClientConfigurationProvider, GEO_API_URL,
    WEATHER_API_URL, WEATHER_DEV_API_URL,
};
#[cfg(feature = "reqwest-http-client")]
mod reqwest;
#[cfg(feature = "reqwest-http-client")]
pub use reqwest::ReqwestHttpAsyncClient;

#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub url: String,
    pub query: HashMap<String, String>,
}

#[async_trait(?Send)]
pub trait AsyncHttpClient {
    type ConfigProvider: AsyncHttpClientConfigurationProvider;
    fn config(&self) -> &Self::ConfigProvider;
    async fn get<T: DeserializeOwned>(&self, req: HttpRequest) -> Result<T>;
}
