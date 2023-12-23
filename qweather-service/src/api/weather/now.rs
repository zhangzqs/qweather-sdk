use anyhow::Result;
use serde::Deserialize;

use crate::common::refer::Refer;
use qweather_http_client::{AHttpClient, HttpRequest};

use super::WeatherInput;

#[derive(Deserialize, Debug, Clone)]
pub struct NowData {
    #[serde(rename = "obsTime")]
    pub obs_time: String,

    #[serde(with = "crate::util::serde::string_to_i32")]
    pub temp: i32,
    #[serde(rename = "feelsLike", with = "crate::util::serde::string_to_i32")]
    pub feels_like: i32,
    pub icon: String,
    pub text: String,
    // pub wind360: u16,
    // pub wind_dir: String,
    // pub wind_scale: u8,
    // pub wind_speed: u8,
    // pub humidity: u8,
    // pub precip: u8,
    // pub pressure: u8,
    // pub vis: u8,
    // pub cloud: u8,
    // pub dew: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NowOutput {
    #[serde(with = "crate::util::serde::string_to_u16")]
    pub code: u16,
    // pub update_time: std::time::Instant,
    #[serde(rename = "fxLink")]
    pub fx_link: String,
    pub now: NowData,
    pub refer: Refer,
}

pub fn now<C: AHttpClient>(client: &C, input: WeatherInput) -> Result<NowOutput> {
    client.get(HttpRequest {
        base_url: "https://api.qweather.com/v7/weather/now".to_string(),
        query: input.to_hash_map(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        const TEST_JSON: &str = include_str!("now_resp_test.json");
        let ret = serde_json::from_str::<NowOutput>(TEST_JSON).unwrap();
        println!("{:#?}", ret);
        assert!(false);
    }
}
