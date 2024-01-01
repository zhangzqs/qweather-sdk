mod api;
pub use api::{
    geo::{CityLookUpInput, CityLookUpOutput, GeoAPI},
    weather::{Weather, WeatherInput},
};

mod common;
pub use common::location::LocationInput;
pub use common::util::{Number, Boolean, UtcDateTime};