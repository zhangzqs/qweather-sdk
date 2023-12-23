use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum LocationInput {
    Text(String),
    LatLong(f64, f64),
}

impl Default for LocationInput {
    fn default() -> Self {
        Self::LatLong(0.0, 0.0)
    }
}

impl From<LocationInput> for String {
    fn from(val: LocationInput) -> Self {
        match val {
            LocationInput::Text(text) => text,
            LocationInput::LatLong(lat, long) => format!("{},{}", lat, long),
        }
    }
}
