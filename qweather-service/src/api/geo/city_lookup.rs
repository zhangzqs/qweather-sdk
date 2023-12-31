use std::collections::HashMap;

use crate::common::location::LocationInput;
use crate::common::util::UtcOffset;
use crate::common::{lang::Lang, refer::Refer};
use crate::{Boolean, Number};
use qweather_http_client::{Api, AsyncHttpClient, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Default)]
pub struct CityLookUpInput {
    pub location: LocationInput,
    pub key: Option<String>,
    pub adm: Option<String>,
    pub range: Option<String>,
    pub number: Option<u8>,
    pub lang: Option<Lang>,
}

impl CityLookUpInput {
    pub(super) fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("location".to_string(), self.location.clone().into());
        if let Some(key) = &self.key {
            map.insert("key".to_string(), key.clone());
        }
        if let Some(adm) = &self.adm {
            map.insert("adm".to_string(), adm.clone());
        }
        if let Some(range) = &self.range {
            map.insert("range".to_string(), range.clone());
        }
        if let Some(number) = &self.number {
            map.insert("number".to_string(), number.to_string());
        }
        if let Some(lang) = &self.lang {
            map.insert("lang".to_string(), lang.clone().into());
        }
        map
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationOutput {
    pub name: String,
    pub id: String,
    pub lat: Number<f32>,
    pub lon: Number<f32>,
    pub adm1: String,
    pub adm2: String,
    pub country: String,
    pub tz: String,
    #[serde(rename = "utcOffset")]
    pub utc_offset: UtcOffset,

    #[serde(rename = "isDst")]
    pub is_dst: Boolean,

    #[serde(rename = "type")]
    pub type_: String,

    pub rank: Number<i32>,

    #[serde(rename = "fxLink")]
    pub fx_link: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CityLookUpOutput {
    pub code: Number<i32>,
    pub location: Option<Vec<LocationOutput>>,
    pub refer: Option<Refer>,
}

pub async fn city_lookup<C: AsyncHttpClient>(
    client: &C,
    input: &CityLookUpInput,
) -> Result<CityLookUpOutput, C::Error> {
    client
        .get(HttpRequest {
            api: Api::Geo,
            path: "/v2/city/lookup".to_string(),
            query: input.to_hash_map(),
        })
        .await
}
