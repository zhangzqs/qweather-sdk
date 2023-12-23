use serde::Deserialize;

use crate::common::refer::Refer;

use super::now::NowData;

enum Hours {
    /// 24 hours
    OneDay,
    /// 72 hours
    ThreeDays,
    /// 168 hours
    SevenDays,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HourlyData {
    #[serde(rename = "fxTime")]
    pub fx_time: String,
    pub temp: String,
    pub icon: String,
    pub text: String,
    #[serde(rename = "wind360")]
    pub wind_360: String,
    #[serde(rename = "windDir")]
    pub wind_dir: String,
    #[serde(rename = "windScale")]
    pub wind_scale: String,
    #[serde(rename = "windSpeed")]
    pub wind_speed: String,
    pub humidity: u8,
    pub pop: Option<u8>,
    pub precip: Option<f32>,
    pub pressure: Option<u8>,
    pub cloud: Option<u8>,
    pub dew: Option<u8>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HourlyOutput {
    pub code: u16,
    // pub update_time: std::time::Instant,
    pub fx_link: String,
    pub now: NowData,
    pub refer: Refer,
}

impl Hours {
    fn to_string(&self) -> String {
        match self {
            Hours::OneDay => "24h".to_string(),
            Hours::ThreeDays => "72h".to_string(),
            Hours::SevenDays => "168h".to_string(),
        }
    }
}