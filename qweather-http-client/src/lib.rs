#[cfg(feature = "reqwest-http-client")]
mod reqwest;
#[cfg(feature = "reqwest-http-client")]
pub use reqwest::ReqwestHttpClient;

use anyhow::Result;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub base_url: String,
    pub query: HashMap<String, String>,
}

pub trait AHttpClient {
    fn set_key(&mut self, key: Option<String>);
    fn get_key(&self) -> Option<String>;
    fn get<T: DeserializeOwned>(&self, req: HttpRequest) -> Result<T>;
}
