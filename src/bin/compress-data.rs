//! ```
//! cargo run --release --bin compress-data
//! ```

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufWriter, Seek, Write},
};

use itertools::Itertools;
use jetlag_map::{
    overpass::{
        ensure::Ensurer,
        models::{Station, StreetElement, StreetWayTags},
    },
    utils::{lat_to_y, lon_to_x},
};

static SF: &'static str =
    "37.70765015159924,-122.5189634289361,37.816301033516325,-122.35772673478483";

#[derive(Debug, Clone, Copy)]
struct StreetNode {
    osm_id: u64,
    index: i32,
    lat: f64,
    lon: f64,
}

impl StreetNode {
    fn with_index(&self, index: i32) -> Self {
        Self { index, ..*self }
    }

    fn x(&self) -> f64 {
        lon_to_x(self.lon)
    }

    fn y(&self) -> f64 {
        lat_to_y(self.lat)
    }
}

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
    // no name: 48170
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

    let mut highway_map = HashMap::new();
    for street in &streets {
        if let StreetElement::Way { tags, .. } = street {
            *highway_map.entry(tags.highway).or_insert(0) += 1;
        }
    }
    // {Cycleway: 568, MotorwayLink: 368, Primary: 1706, SecondaryLink: 155,
    // Service: 8235, LivingStreet: 22, Platform: 184, Construction: 33, Track:
    // 46, Path: 902, Secondary: 2364, Corridor: 92, Unclassified: 394,
    // Motorway: 300, Trunk: 293, Elevator: 16, Tertiary: 2259, ResidentialLink:
    // 4, Pedestrian: 369, BusStop: 3, Busway: 165, Residential: 5908,
    // TertiaryLink: 77, TrunkLink: 31, PrimaryLink: 233, Proposed: 2,
    // Bridleway: 1, Steps: 2500, Footway: 36188}
    eprintln!("{highway_map:?}");

    let referenced_nodes = streets
        .iter()
        .filter_map(|element| match element {
            StreetElement::Way { nodes, .. } => Some(nodes),
            _ => None,
        })
        .flat_map(|nodes| nodes)
        .collect::<HashSet<_>>();

    let mut node_coords = streets
        .iter()
        .filter_map(|element| match element {
            StreetElement::Node { id, lat, lon } => Some(StreetNode {
                osm_id: *id,
                lat: *lat,
                lon: *lon,
                index: 0,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    // Sorting by longitude then latitude brings max abs node diff down
    // under i16::MAX
    node_coords.sort_by(|a, b| {
        a.lon
            .total_cmp(&b.lon)
            .then_with(|| a.lat.total_cmp(&b.lat))
    });
    let lat_extremes = node_coords
        .iter()
        .map(|node| node.lat)
        .minmax()
        .into_option()
        .expect("should be at least one node");
    let lon_extremes = node_coords
        .iter()
        .map(|node| node.lon)
        .minmax()
        .into_option()
        .expect("should be at least one node");
    // (37.6876263, 37.8323305) (-122.514537, -122.3276982)
    eprintln!("{lat_extremes:?} {lon_extremes:?}");

    let node_index_map = node_coords
        .iter()
        .filter_map(|entry| {
            if referenced_nodes.contains(&entry.osm_id) {
                Some(entry)
            } else {
                None
            }
        })
        .zip(0i32..)
        .map(|(node, index)| (node.osm_id, node.with_index(index)))
        .collect::<HashMap<_, _>>();
    // 222561 / 222561 nodes (max: 65535)
    eprintln!(
        "{} / {} nodes (max: {})",
        node_index_map.len(),
        node_coords.len(),
        u16::MAX
    );

    {
        let streets_optimized = File::create("src/streets_optimized.bin")?;
        let mut writer = BufWriter::new(streets_optimized);

        writer.write_all(&lat_extremes.0.to_be_bytes())?;
        writer.write_all(&lat_extremes.1.to_be_bytes())?;
        writer.write_all(&lon_extremes.0.to_be_bytes())?;
        writer.write_all(&lon_extremes.1.to_be_bytes())?;
        writer.write_all(&TryInto::<u32>::try_into(node_coords.len())?.to_be_bytes())?;

        for StreetNode { lat, lon, .. } in &node_coords {
            writer.write_all(
                &(((*lat - lat_extremes.0) / (lat_extremes.1 - lat_extremes.0) * (u16::MAX as f64))
                    as u16)
                    .to_be_bytes(),
            )?;
            writer.write_all(
                &(((*lon - lon_extremes.0) / (lon_extremes.1 - lon_extremes.0) * (u16::MAX as f64))
                    as u16)
                    .to_be_bytes(),
            )?;
        }

        let nodes_size = writer.stream_position()?;
        // nodes take up 890280 bytes
        eprintln!("nodes take up {nodes_size} bytes");

        let mut way_count = 0;
        let mut way_node_count = 0;
        for street in &streets {
            let StreetElement::Way { nodes, .. } = street else {
                continue;
            };
            way_count += 1;
            way_node_count += nodes.len();
            writer.write_all(&TryInto::<u16>::try_into(nodes.len())?.to_be_bytes())?;
            writer.write_all(
                &node_index_map
                    .get(&&nodes[0])
                    .expect("missing first node id")
                    .index
                    .to_be_bytes(),
            )?;
            for (prev, index) in nodes
                .iter()
                .map(|node_id| node_index_map.get(&node_id).expect("missing node id").index)
                .tuple_windows()
            {
                writer.write_all(&TryInto::<i16>::try_into(index - prev)?.to_be_bytes())?;
            }
        }
        eprintln!("btw there are {way_count} ways, {way_node_count}ish segments total");

        writer.flush()?;

        let total_size = writer.stream_position()?;
        // wrote src/streets_optimized.bin (1797250 bytes, ways was 906970 bytes)
        eprintln!(
            "wrote src/streets_optimized.bin ({} bytes, ways was {} bytes)",
            total_size,
            total_size - nodes_size
        );
    }
    {
        let streets_optimized2 = File::create("src/streets_optimized2.bin")?;
        let mut writer2 = BufWriter::new(streets_optimized2);

        for street in &streets {
            let StreetElement::Way { nodes, .. } = street else {
                continue;
            };
            writer2.write_all(&TryInto::<u16>::try_into(nodes.len())?.to_be_bytes())?;
            for StreetNode { lat, lon, .. } in nodes
                .iter()
                .map(|node_id| node_index_map.get(&node_id).expect("missing node id"))
            {
                writer2.write_all(
                    &(((*lat - lat_extremes.0) / (lat_extremes.1 - lat_extremes.0)
                        * (u16::MAX as f64)) as u16)
                        .to_be_bytes(),
                )?;
                writer2.write_all(
                    &(((*lon - lon_extremes.0) / (lon_extremes.1 - lon_extremes.0)
                        * (u16::MAX as f64)) as u16)
                        .to_be_bytes(),
                )?;
            }
        }

        writer2.flush()?;
        // wrote src/streets_optimized2.bin (1433432 bytes)
        eprintln!(
            "wrote src/streets_optimized2.bin ({} bytes)",
            writer2.stream_position()?
        );
    }

    let x_extremes = node_coords
        .iter()
        .map(|node| node.x())
        .minmax()
        .into_option()
        .expect("should be at least one node");
    let y_extremes = node_coords
        .iter()
        .map(|node| node.y())
        .minmax()
        .into_option()
        .expect("should be at least one node");
    // x=(-0.6806363166666667, -0.6795983233333334) y=(-0.22736193843536473, -0.2263450777733419)
    eprintln!("x={x_extremes:?} y={y_extremes:?}");
    // width=0.0010379933333333202 height=0.0010168606620228338
    eprintln!(
        "width={} height={}",
        x_extremes.1 - x_extremes.0,
        y_extremes.1 - y_extremes.0
    );

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
