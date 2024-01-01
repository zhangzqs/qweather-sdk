#[cfg(feature = "reqwest-http-client")]
mod reqwest;
#[cfg(feature = "reqwest-http-client")]
pub use reqwest::ReqwestHttpClient;

use anyhow::Result;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub url: String,
    pub query: HashMap<String, String>,
}

pub struct StaticHttpClientConfigurationProvider {
    pub key: Option<String>,
}

impl HttpClientConfigurationProvider for StaticHttpClientConfigurationProvider {
    fn get_key(&self) -> Option<&str> {
        if let Some(ref key) = self.key {
            Some(key)
        } else {
            None
        }
    }
}

pub trait HttpClientConfigurationProvider {
    fn get_key(&self) -> Option<&str>;
}

pub trait AHttpClient {
    type Configuration: HttpClientConfigurationProvider;
    fn get<T: DeserializeOwned>(&self, req: HttpRequest) -> Result<T>;
}
