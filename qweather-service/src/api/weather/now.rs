use anyhow::Result;
use serde::Deserialize;

use crate::{common::refer::Refer, Number, UtcDateTime};
use qweather_http_client::{Api, AsyncHttpClient, HttpRequest};

use super::WeatherInput;

#[derive(Deserialize, Debug, Clone)]
pub struct NowData {
    /// 数据观测时间
    #[serde(rename = "obsTime")]
    pub obs_time: UtcDateTime,

    /// 温度，默认单位：摄氏度
    pub temp: Number<i32>,

    /// 体感温度，单位摄氏度
    #[serde(rename = "feelsLike")]
    pub feels_like: Number<i32>,

    /// 天气图标代码
    pub icon: String,

    /// 天气状况文字描述
    pub text: String,

    /// 风向的360角度
    pub wind360: Number<f32>,

    /// 风向
    #[serde(rename = "windDir")]
    pub wind_dir: String,

    /// 风力等级
    #[serde(rename = "windScale")]
    pub wind_scale: Number<i32>,

    /// 风速
    #[serde(rename = "windSpeed")]
    pub wind_speed: Number<f32>,

    /// 相对湿度，百分比数值
    pub humidity: Number<f32>,

    /// 当前小时累计降水量，默认单位：毫米
    pub precip: Number<f32>,

    /// 大气压强，默认单位：百帕
    pub pressure: Number<f32>,

    /// 能见度，默认单位：公里
    pub vis: Number<f32>,

    /// 云量，百分比数值。可能为空
    pub cloud: Option<Number<f32>>,

    /// 露点温度。可能为空
    pub dew: Option<Number<f32>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NowOutput {
    pub code: Number<i32>,
    #[serde(rename = "updateTime")]
    pub update_time: UtcDateTime,
    #[serde(rename = "fxLink")]
    pub fx_link: String,
    pub now: NowData,
    pub refer: Refer,
}

pub async fn now<C: AsyncHttpClient>(client: &C, input: &WeatherInput) -> Result<NowOutput> {
    client
        .get(HttpRequest {
            api: Api::Weather,
            path: "/v7/weather/now".into(),
            query: input.to_hash_map(),
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        const TEST_JSON: &str = include_str!("now_resp_test.json");
        let ret = serde_json::from_str::<NowOutput>(TEST_JSON).unwrap();
        println!("{:#?}", ret);
    }
}
