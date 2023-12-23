pub mod serde {
    pub mod string_to_i32 {
        use serde::{Deserialize, Deserializer};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse::<i32>().map_err(serde::de::Error::custom)
        }
    }

    pub mod string_to_u16 {
        use serde::{Deserialize, Deserializer};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse::<u16>().map_err(serde::de::Error::custom)
        }
    }

    pub mod string_to_f64 {
        use serde::{Deserialize, Deserializer};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse::<f64>().map_err(serde::de::Error::custom)
        }
    }

    pub mod string_to_bool {
        use serde::{Deserialize, Deserializer};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "0" | "false" => Ok(false),
                "1" | "true" => Ok(true),
                _ => Err(serde::de::Error::custom(format!(
                    "cannot parse as boolean: {}",
                    s
                ))),
            }
        }
    }
}
