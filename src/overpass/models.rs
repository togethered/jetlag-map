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

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreetElement {
    Node {
        id: u64,
        lat: f64,
        lon: f64,
    },
    Way {
        id: u64,
        nodes: Vec<u64>,
        tags: StreetWayTags,
    },
}

#[derive(Debug, Deserialize)]
pub struct StreetWayTags {
    pub highway: StreetWayType,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StreetWayType {
    Footway,
    Residential,
    Motorway,
    Service,
    Unclassified,
    Tertiary,
    Secondary,
    Steps,
    PrimaryLink,
    MotorwayLink,
    Cycleway,
    TertiaryLink,
    TrunkLink,
    SecondaryLink,
    Primary,
    ResidentialLink,
    Path,
    Pedestrian,
    Trunk,
    LivingStreet,
    Construction,
    Track,
    Busway,
    Platform,
    Corridor,
    Bridleway,
    Proposed,
    Elevator,
    BusStop,
}
