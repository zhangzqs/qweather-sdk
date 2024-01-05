use anyhow::Result;
use async_trait::async_trait;

pub struct ReqwestHttpAsyncClientConfiguration {
    pub key: String,
    pub weather_base_url: Option<String>,
    pub geo_base_url: Option<String>,
}

impl Default for ReqwestHttpAsyncClientConfiguration {
    fn default() -> Self {
        Self {
            key: "".into(),
            weather_base_url: None,
            geo_base_url: None,
        }
    }
}

pub struct ReqwestHttpAsyncClient {
    client: reqwest::Client,
    key: String,
    weather_base_url: Option<String>,
    geo_base_url: Option<String>,
}

impl ReqwestHttpAsyncClient {
    pub fn new(conf: &ReqwestHttpAsyncClientConfiguration) -> Result<Self> {
        Ok(Self {
            client: reqwest::ClientBuilder::new().gzip(true).build()?,
            key: conf.key.clone(),
            weather_base_url: conf.weather_base_url.clone(),
            geo_base_url: conf.geo_base_url.clone(),
        })
    }
}

impl super::AsyncHttpClient for ReqwestHttpAsyncClient {
    fn get<T: serde::de::DeserializeOwned>(
        &self,
        req: crate::HttpRequest,
    ) -> impl std::future::Future<Output = Result<T>> + Send {
        let mut query = req.query;
        if !query.contains_key("key") {
            query.insert("key".into(), self.key.clone());
        }

        let url = match req.api {
            crate::Api::Geo => {
                if let Some(url) = &self.geo_base_url {
                    format!("{}{}", url, req.path)
                } else {
                    format!("{}{}", crate::GEO_API_URL, req.path)
                }
            }
            crate::Api::Weather => {
                if let Some(url) = &self.weather_base_url {
                    format!("{}{}", url, req.path)
                } else {
                    format!("{}{}", crate::WEATHER_DEV_API_URL, req.path)
                }
            }
        };

        let req_builder = self.client.get(url).query(&query);
        async move { Ok(req_builder.send().await?.json::<T>().await?) }
    }
}
