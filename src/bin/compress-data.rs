//! ```
//! cargo run --bin compress-data
//! ```

use std::{collections::HashMap, error::Error};

use jetlag_map::overpass::{ensure::Ensurer, models::Station};

static SF: &'static str =
    "37.70765015159924,-122.5189634289361,37.816301033516325,-122.35772673478483";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Ensurer::new();
    client.ensure(
        "data/train-lines.json",
        &format!(
            r#"[bbox:{SF}][out:json];
            relation["type"="route"]["route"~"train|subway|light_rail"];
            out geom;"#
        ),
    )?;
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
        .read::<Station>()?;
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
        "data/streets.json",
        &format!(
            r#"[bbox:{SF}][out:json];
            way["highway"];
            out body;
            >;
            out skel qt;"#
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

    println!(
        "{:?}",
        stations
            .elements
            .iter()
            .filter(|station| station.tags.network.is_none())
            .map(|station| &station.tags.name)
            .collect::<Vec<_>>()
    );

    let mut networks = HashMap::new();
    for station in stations.elements {
        if let Some(network) = station.tags.network {
            *networks.entry(network).or_insert(0) += 1;
        }
    }
    println!("{networks:?}");

    Ok(())
}
