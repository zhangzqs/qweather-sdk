use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum LocationInput {
    Text(String),
    ID(String),
    LatLong(f32, f32),
}

impl Default for LocationInput {
    fn default() -> Self {
        Self::LatLong(0.0, 0.0)
    }
}

impl From<LocationInput> for String {
    fn from(val: LocationInput) -> Self {
        use LocationInput::{LatLong, Text, ID};
        match val {
            Text(text) | ID(text) => text,
            LatLong(lat, long) => format!("{},{}", lat, long),
        }
    }
}
