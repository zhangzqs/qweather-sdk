use anyhow::{Ok, Result};
use qweather_http_client::{
    ReqwestHttpClient, StaticHttpClientConfigurationProvider, WEATHER_DEV_API_URL,
};
use qweather_service::{CityLookUpInput, GeoAPI, LocationInput, Weather, WeatherInput};

const KEY: &str = include_str!("../key");
fn main() -> Result<()> {
    // std::env::set_var("RUSTLOG", "debug");
    env_logger::init();
    log::debug!("Hello");
    let client = ReqwestHttpClient::new(StaticHttpClientConfigurationProvider {
        key: Some(KEY),
        weather_base_url: Some(WEATHER_DEV_API_URL),
        ..Default::default()
    })?;
    let geo = GeoAPI::new(&client);
    let ret = geo.city_lookup(&CityLookUpInput {
        location: LocationInput::Text("浦东".to_string()),
        ..Default::default()
    })?;
    println!("{:#?}", ret);
    let output = &ret.location[0];

    let weather = Weather::new(&client);
    let w = weather.now(&WeatherInput {
        location: LocationInput::ID(output.id.clone()),
        ..Default::default()
    })?;
    println!("{:#?}", w);
    Ok(())
}
