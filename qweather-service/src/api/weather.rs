use std::collections::HashMap;

use crate::common::{lang::Lang, location::LocationInput, unit::Unit};
use qweather_http_client::AsyncHttpClient;

use self::now::NowOutput;

#[derive(Debug, Clone, Default)]

pub struct WeatherInput {
    pub location: LocationInput,
    pub key: Option<String>,
    pub lang: Option<Lang>,
    pub unit: Option<Unit>,
}

impl WeatherInput {
    pub(self) fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("location".to_string(), self.location.clone().into());
        if let Some(key) = &self.key {
            map.insert("key".to_string(), key.clone());
        }
        if let Some(lang) = &self.lang {
            map.insert("lang".to_string(), lang.clone().into());
        }
        if let Some(unit) = &self.unit {
            let unit = (*unit).into();
            map.insert("unit".to_string(), unit);
        }
        map
    }
}

// mod daily;
// mod hourly;
mod now;

pub struct Weather<'a, C: AsyncHttpClient> {
    client: &'a C,
}

impl<'a, C: AsyncHttpClient> Weather<'a, C> {
    pub fn new(client: &'a C) -> Weather<'a, C> {
        Self { client }
    }

    pub async fn now(&self, input: &WeatherInput) -> Result<NowOutput, C::Error> {
        now::now(self.client, input).await
    }
}
