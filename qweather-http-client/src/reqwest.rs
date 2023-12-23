use std::collections::HashMap;

use anyhow::{Ok, Result};
use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::AHttpClient;

pub struct ReqwestHttpClient {
    key: Option<String>,
    client: reqwest::blocking::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: reqwest::blocking::ClientBuilder::new().gzip(true).build()?,
            key: None,
        })
    }
}

impl AHttpClient for ReqwestHttpClient {
    fn get<T: DeserializeOwned>(&self, req: crate::HttpRequest) -> anyhow::Result<T> {
        let mut query = req.query;
        if !query.contains_key("key") && self.key.is_some() {
            query.insert("key".into(), self.get_key().unwrap().into());
        }
        Ok(self
            .client
            .get(req.base_url)
            .query(&query)
            .send()?
            .json::<T>()?)
    }

    fn set_key(&mut self, key: Option<String>) {
        self.key = key;
    }

    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }
}
