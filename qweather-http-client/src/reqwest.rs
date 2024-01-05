use log::debug;

use crate::{GEO_API_URL, WEATHER_DEV_API_URL};

pub struct ReqwestHttpAsyncClientConfiguration {
    pub key: String,
    pub weather_base_url: Option<String>,
    pub geo_base_url: Option<String>,
    pub timeout: Option<std::time::Duration>,
    pub debug: bool,
}

impl Default for ReqwestHttpAsyncClientConfiguration {
    fn default() -> Self {
        Self {
            key: "".into(),
            weather_base_url: None,
            geo_base_url: None,
            timeout: None,
            debug: false,
        }
    }
}

pub struct ReqwestHttpAsyncClient {
    client: reqwest::Client,
    key: String,
    weather_base_url: String,
    geo_base_url: String,
}

impl ReqwestHttpAsyncClient {
    pub fn new(conf: &ReqwestHttpAsyncClientConfiguration) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: {
                let cli = reqwest::ClientBuilder::new().gzip(true);
                let cli = if let Some(timeout) = conf.timeout {
                    cli.timeout(timeout)
                } else {
                    cli
                };
                cli.build()?
            },
            key: conf.key.clone(),
            weather_base_url: conf
                .weather_base_url
                .clone()
                .unwrap_or_else(|| WEATHER_DEV_API_URL.into()),
            geo_base_url: conf
                .geo_base_url
                .clone()
                .unwrap_or_else(|| GEO_API_URL.into()),
        })
    }
}

impl super::AsyncHttpClient for ReqwestHttpAsyncClient {
    type Error = reqwest::Error;
    fn get<T: serde::de::DeserializeOwned>(
        &self,
        req: crate::HttpRequest,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send {
        let mut query = req.query;
        if !query.contains_key("key") {
            query.insert("key".into(), self.key.clone());
        }

        let url = format!(
            "{}{}",
            match req.api {
                crate::Api::Geo => &self.geo_base_url,
                crate::Api::Weather => &self.weather_base_url,
            },
            req.path
        );

        debug!("url: {}, query: {:?}", url, query);
        let req_builder = self.client.get(url).query(&query);
        async move { req_builder.send().await?.json::<T>().await }
    }
}
