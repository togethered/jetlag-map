use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OverpassResult<T> {
    pub elements: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Station {
    pub lat: f64,
    pub lon: f64,
    pub tags: StationTags,
}

#[derive(Debug, Deserialize)]
pub struct StationTags {
    pub name: String,
    /// `None` in only one case (Red & White Fleet), which is a ferry station
    pub network: Option<String>,
}
