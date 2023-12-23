#[derive(Debug, Clone, Copy)]
pub enum Unit {
    Metric,
    Imperial,
}

impl From<Unit> for String {
    fn from(val: Unit) -> Self {
        match val {
            Unit::Metric => "m".to_string(),
            Unit::Imperial => "i".to_string(),
        }
    }
}