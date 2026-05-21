//! ```
//! cargo run --release --bin compress-data
//! ```

use std::{collections::HashMap, error::Error};

use itertools::Itertools;
use jetlag_map::overpass::{
    ensure::Ensurer,
    models::{Station, StreetElement, StreetWayTags},
};

static SF: &'static str =
    "37.70765015159924,-122.5189634289361,37.816301033516325,-122.35772673478483";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Ensurer::new();

    let stations = client
        .ensure(
            "data/train-stations.json",
            &format!(
                r#"[bbox:{SF}][out:json];
            (
                node["railway"="station"];
                node["railway"="tram_stop"];
                node["station"="subway"];
                node["public_transport"="station"];
            );
            out body;"#
            ),
        )?
        .read::<Station>()?
        .elements;

    // ["Red & White Fleet"]
    eprintln!(
        "{:?}",
        stations
            .iter()
            .filter(|station| station.tags.network.is_none())
            .map(|station| &station.tags.name)
            .collect::<Vec<_>>()
    );

    let mut networks = HashMap::new();
    for station in stations {
        if let Some(network) = station.tags.network {
            *networks.entry(network).or_insert(0) += 1;
        }
    }
    // {"BART": 8, "Muni": 359, "Caltrain": 3, "San Francisco Bay Ferry": 2,
    // "Muni;GGT;AC Transit;WestCAT;Greyhound;Flixbus": 1, "GGT;Muni;PresidiGo":
    // 1, "PresidiGo;Muni": 1, "Tahoe Convoy": 1}
    eprintln!("{networks:?}");

    let streets = client
        .ensure(
            "data/streets.json",
            &format!(
                r#"[bbox:{SF}][out:json];
            way["highway"];
            out body;
            >;
            out skel qt;"#
            ),
        )?
        .read::<StreetElement>()?
        .elements;

    // 285979
    eprintln!("{}", streets.len());
    eprintln!(
        "no name: {}",
        streets
            .iter()
            .filter(|street| match street {
                StreetElement::Way {
                    tags: StreetWayTags { name: None, .. },
                    ..
                } => true,
                _ => false,
            })
            .count()
    );

    let mut highway = HashMap::new();
    for street in &streets {
        if let StreetElement::Way { tags, .. } = street {
            *highway.entry(tags.highway).or_insert(0) += 1;
        }
    }
    // {Cycleway: 568, MotorwayLink: 368, Primary: 1706, SecondaryLink: 155,
    // Service: 8235, LivingStreet: 22, Platform: 184, Construction: 33, Track:
    // 46, Path: 902, Secondary: 2364, Corridor: 92, Unclassified: 394,
    // Motorway: 300, Trunk: 293, Elevator: 16, Tertiary: 2259, ResidentialLink:
    // 4, Pedestrian: 369, BusStop: 3, Busway: 165, Residential: 5908,
    // TertiaryLink: 77, TrunkLink: 31, PrimaryLink: 233, Proposed: 2,
    // Bridleway: 1, Steps: 2500, Footway: 36188}
    eprintln!("{highway:?}");

    let nodes = streets
        .iter()
        .filter_map(|element| match element {
            StreetElement::Node { id, lat, lon } => Some((id, (lat, lon))),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let lat_extremes = nodes
        .values()
        .map(|node| node.0)
        .minmax()
        .into_option()
        .expect("should be at least one node");
    let lon_extremes = nodes
        .values()
        .map(|node| node.1)
        .minmax()
        .into_option()
        .expect("should be at least one node");
    eprintln!("{lat_extremes:?} {lon_extremes:?}");

    let ways = streets
        .iter()
        .filter_map(|element| match element {
            StreetElement::Way { nodes, .. } => Some(nodes),
            _ => None,
        })
        .map(|node_ids| {
            node_ids
                .iter()
                .filter_map(|node_id| nodes.get(node_id))
                .collect_vec()
        })
        .collect_vec();

    client.ensure(
        "data/train-lines.json",
        &format!(
            r#"[bbox:{SF}][out:json];
            relation["type"="route"]["route"~"train|subway|light_rail"];
            out geom;"#
        ),
    )?;
    client.ensure(
        "data/poi.json",
        &format!(
            r#"[bbox:{SF}][out:json];
            (
                node["tourism"="zoo"];
                way["tourism"="zoo"];
                node["tourism"="aquarium"];
                way["tourism"="aquarium"];
                node["amenity"="library"];
                way["amenity"="library"];
                node["tourism"="museum"];
                way["tourism"="museum"];
                node["amenity"="hospital"];
                way["amenity"="hospital"];
            );
            out center;"#
        ),
    )?;
    client.ensure(
        "data/city-limits.json",
        &format!(
            r#"[bbox:{SF}][out:json];
            way["natural"="coastline"];
            out geom;"#
        ),
    )?;

    Ok(())
}
