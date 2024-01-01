use serde::Deserialize;

use crate::common::refer::Refer;

enum Days {
    Three,
    Seven,
    Ten,
    Fifteen,
    Thirty,
}

impl Days {
    fn to_path_args(&self) -> String {
        match self {
            Days::Three => "3d",
            Days::Seven => "7d",
            Days::Ten => "10d",
            Days::Fifteen => "15d",
            Days::Thirty => "30d",
        }
        .to_string()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyData {
    // #[serde(rename = "fxDate", with = "date_format")]
    // pub fx_date: NaiveDate,
    // #[serde(with = "time_format")]
    // pub sunrise: Option<NaiveTime>,
    // #[serde(with = "time_format")]
    // pub sunset: Option<NaiveTime>,
    // #[serde(with = "time_format")]
    // pub moonrise: Option<NaiveTime>,
    // #[serde(with = "time_format")]
    // pub moonset: Option<NaiveTime>,
    #[serde(rename = "moonPhase")]
    pub moon_phase: String,
    #[serde(rename = "moonPhaseIcon")]
    pub moon_phase_icon: String,
    #[serde(rename = "tempMax")]
    pub temp_max: i32,
    #[serde(rename = "tempMin")]
    pub temp_min: i32,
    #[serde(rename = "iconDay")]
    pub icon_day: String,
    #[serde(rename = "textDay")]
    pub text_day: String,
    #[serde(rename = "iconNight")]
    pub icon_night: String,
    #[serde(rename = "textNight")]
    pub text_night: String,
    #[serde(rename = "wind360Day")]
    pub wind_360_day: i32,
    #[serde(rename = "windDirDay")]
    pub wind_dir_day: String,
    #[serde(rename = "windScaleDay")]
    pub wind_scale_day: String,
    #[serde(rename = "windSpeedDay")]
    pub wind_speed_day: i32,
    #[serde(rename = "wind360Night")]
    pub wind_360_night: i32,
    #[serde(rename = "windDirNight")]
    pub wind_dir_night: String,
    #[serde(rename = "windScaleNight")]
    pub wind_scale_night: String,
    #[serde(rename = "windSpeedNight")]
    pub wind_speed_night: i32,
    pub humidity: i32,
    pub precip: f32,
    pub pressure: i32,
    pub vis: i32,
    pub cloud: i32,
    #[serde(rename = "uvIndex")]
    pub uv_index: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HourlyOutput {
    pub code: u16,
    // pub update_time: std::time::Instant,
    pub fx_link: String,
    pub daily: DailyData,
    pub refer: Refer,
}
