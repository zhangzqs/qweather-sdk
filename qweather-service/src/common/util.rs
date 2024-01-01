use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use time_macros::format_description;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Number<T>(T);

impl<'de, T: FromStr<Err = E>, E: Display> Deserialize<'de> for Number<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(
            String::deserialize(deserializer)?
                .parse::<T>()
                .map_err(serde::de::Error::custom)?,
        ))
    }
}

impl<T: FromStr + ToString> ToString for Number<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T> Number<T> {
    pub fn take(self) -> T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Boolean(bool);

impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(match s.as_str() {
            "0" | "false" => Ok(false),
            "1" | "true" => Ok(true),
            _ => Err(serde::de::Error::custom(format!(
                "cannot parse as boolean: {}",
                s
            ))),
        }?))
    }
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UtcDateTime(OffsetDateTime);

impl<'de> Deserialize<'de> for UtcDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = OffsetDateTime::parse(&s, &Iso8601::DATE_TIME_OFFSET)
            .map_err(serde::de::Error::custom)?;
        Ok(Self(date))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UtcOffset(time::UtcOffset);

impl<'de> Deserialize<'de> for UtcOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let fmt = format_description!("[offset_hour]:[offset_minute]");
        let t = time::UtcOffset::parse(&s, fmt).map_err(serde::de::Error::custom)?;
        Ok(Self(t))
    }
}
