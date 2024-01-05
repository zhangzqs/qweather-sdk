use anyhow::{Ok, Result};
use qweather_http_client::{ReqwestHttpAsyncClient, ReqwestHttpAsyncClientConfiguration};
use qweather_service::{CityLookUpInput, GeoApi, LocationInput, Weather, WeatherInput};

const KEY: &str = include_str!("../key");

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    log::debug!("Hello");
    let client = ReqwestHttpAsyncClient::new(&ReqwestHttpAsyncClientConfiguration {
        key: KEY.to_string(),
        ..Default::default()
    })?;
    let geo = GeoApi::new(&client);
    let ret = geo
        .city_lookup(&CityLookUpInput {
            location: LocationInput::Text("浦东".to_string()),
            ..Default::default()
        })
        .await?;
    println!("{:#?}", ret);
    let output = &ret.location[0];

    let weather = Weather::new(&client);
    let w = weather
        .now(&WeatherInput {
            location: LocationInput::ID(output.id.clone()),
            ..Default::default()
        })
        .await?;
    println!("{:#?}", w);
    Ok(())
}
