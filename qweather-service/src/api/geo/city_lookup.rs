use std::collections::HashMap;

use crate::common::location::LocationInput;
use crate::common::{lang::Lang, refer::Refer};
use anyhow::Result;
use qweather_http_client::{AHttpClient, HttpRequest};
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
    #[serde(with = "crate::util::serde::string_to_f64")]
    pub lat: f64,
    #[serde(with = "crate::util::serde::string_to_f64")]
    pub lon: f64,
    pub adm1: String,
    pub adm2: String,
    pub country: String,
    pub tz: String,
    #[serde(rename = "utcOffset")]
    pub utc_offset: String,

    #[serde(rename = "isDst", with = "crate::util::serde::string_to_bool")]
    pub is_dst: bool,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(with = "crate::util::serde::string_to_u16")]
    pub rank: u16,

    #[serde(rename = "fxLink")]
    pub fx_link: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CityLookUpOutput {
    #[serde(with = "crate::util::serde::string_to_u16")]
    pub code: u16,
    pub location: Vec<LocationOutput>,
    pub refer: Refer,
}

fn city_lookup<C: AHttpClient>(client: &C, input: CityLookUpInput) -> Result<CityLookUpOutput> {
    client.get(HttpRequest {
        base_url: "https://geoapi.qweather.com/v2/city/lookup".to_string(),
        query: input.to_hash_map(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_lookup() {
        const TEST_JSON: &str = include_str!("city_lookup_resp_test.json");
        let ret = serde_json::from_str::<CityLookUpOutput>(TEST_JSON).unwrap();
        assert_eq!(ret.code, 200);
        assert_eq!(ret.location.len(), 10);
        assert_eq!(ret.location[0].name, "北京");
        assert_eq!(ret.location[0].id, "101010100");
        assert_eq!(ret.location[0].lat, 39.90499);
        assert_eq!(ret.location[0].lon, 116.40529);
        assert_eq!(ret.location[0].adm1, "北京市");
        assert_eq!(ret.location[0].adm2, "北京");
        assert_eq!(ret.location[0].country, "中国");
        assert_eq!(ret.location[0].tz, "Asia/Shanghai");
        assert_eq!(ret.location[0].utc_offset, "+08:00");
        assert!(!ret.location[0].is_dst);
        assert_eq!(ret.location[0].type_, "city");
        assert_eq!(ret.location[0].rank, 10);
        assert_eq!(
            ret.location[0].fx_link,
            "https://www.qweather.com/weather/beijing-101010100.html"
        );
        println!("{:?}", ret);
    }
}
