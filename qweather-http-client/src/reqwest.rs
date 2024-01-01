use anyhow::{Ok, Result};

use serde::de::DeserializeOwned;

use crate::{AHttpClient, HttpClientConfigurationProvider};

pub struct ReqwestHttpClient<C> {
    config_provider: C,
    client: reqwest::blocking::Client,
}

impl<C: HttpClientConfigurationProvider> ReqwestHttpClient<C> {
    pub fn new(config_provider: C) -> Result<Self> {
        Ok(Self {
            client: reqwest::blocking::ClientBuilder::new().gzip(true).build()?,
            config_provider,
        })
    }
}

impl<C: HttpClientConfigurationProvider> AHttpClient for ReqwestHttpClient<C> {
    type ConfigProvider = C;
    fn config(&self) -> &Self::ConfigProvider {
        &self.config_provider
    }
    fn get<T: DeserializeOwned>(&self, req: crate::HttpRequest) -> anyhow::Result<T> {
        let req_builder = self.client.get(req.url).query(&{
            let mut query = req.query;
            let key = self.config_provider.key();
            if !query.contains_key("key") && key.is_some() {
                query.insert("key".into(), key.unwrap().into());
            }
            query
        });
        if let Some(r) = req_builder.try_clone() {
            let t = r.send()?;
            println!("url: {}", t.url());
            println!("debug: {}", t.text()?);
        }
        Ok(req_builder.send()?.json::<T>()?)
    }
}
