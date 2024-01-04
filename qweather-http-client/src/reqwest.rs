use crate::config::AsyncHttpClientConfigurationProvider;
use anyhow::Result;
use async_trait::async_trait;
pub struct ReqwestHttpAsyncClient<C> {
    config_provider: C,
    client: reqwest::Client,
}

impl<C: AsyncHttpClientConfigurationProvider> ReqwestHttpAsyncClient<C> {
    pub fn new(config_provider: C) -> Result<Self> {
        Ok(Self {
            client: reqwest::ClientBuilder::new().gzip(true).build()?,
            config_provider,
        })
    }
}

#[async_trait(?Send)]
impl<C: AsyncHttpClientConfigurationProvider+Send> super::AsyncHttpClient for ReqwestHttpAsyncClient<C> {
    type ConfigProvider = C;
    fn config(&self) -> &Self::ConfigProvider {
        &self.config_provider
    }
    async fn get<T: serde::de::DeserializeOwned>(&self, req: crate::HttpRequest) -> Result<T> {
        let mut query = req.query;
        let key = self.config_provider.key().await;
        if !query.contains_key("key") && key.is_some() {
            query.insert("key".into(), key.unwrap().into());
        }
        let req_builder = self.client.get(req.url).query(&query);
        if let Some(r) = req_builder.try_clone() {
            let t = r.send().await?;
            println!("url: {}", t.url());
            println!("debug: {}", t.text().await?);
        }
        Ok(req_builder.send().await?.json::<T>().await?)
    }
}
