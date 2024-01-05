use serde::de::DeserializeOwned;
use std::{collections::HashMap, future::Future};

#[cfg(feature = "reqwest-http-client")]
mod reqwest;
#[cfg(feature = "reqwest-http-client")]
pub use reqwest::{ReqwestHttpAsyncClient, ReqwestHttpAsyncClientConfiguration};

pub static GEO_API_URL: &str = "https://geoapi.qweather.com";
pub static WEATHER_API_URL: &str = "https://api.qweather.com";
pub static WEATHER_DEV_API_URL: &str = "https://devapi.qweather.com";

#[derive(Debug, Clone, Default)]
pub enum Api {
    Geo,
    #[default]
    Weather,
}

#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    pub api: Api,
    pub path: String,
    pub query: HashMap<String, String>,
}

pub trait AsyncHttpClient {
    type Error;
    fn get<T: DeserializeOwned>(
        &self,
        req: HttpRequest,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;
}
