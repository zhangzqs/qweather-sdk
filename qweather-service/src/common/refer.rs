use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Refer {
    pub sources: Vec<String>,
    pub license: Vec<String>,
}
