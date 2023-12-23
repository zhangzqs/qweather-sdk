use anyhow::{Ok, Result};
use qweather_http_client::{AHttpClient, ReqwestHttpClient};
use qweather_service::{CityLookUpInput, GeoAPI, LocationInput};

const KEY: &str = include_str!("../key");
fn main() -> Result<()> {
    let mut client = ReqwestHttpClient::new()?;
    client.set_key(Some(KEY.to_string()));
    let geo = GeoAPI::new(&client);
    let ret = geo.city_lookup(&CityLookUpInput {
        location: LocationInput::Text("上海".to_string()),
        ..Default::default()
    })?;
    println!("{:#?}", ret);
    Ok(())
}
