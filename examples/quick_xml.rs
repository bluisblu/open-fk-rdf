use open_fk_rdf::decode_rdf;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fs::File, io::{self, Read}};

// Incomplete implementation of profile.rdf
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "profile")]
#[skip_serializing_none]
struct Profile {
    #[serde(rename = "@coconuts")]
    coconuts: Option<u32>,
    #[serde(rename = "@gname")]
    g_name: Option<String>,
    #[serde(rename = "@savepassword")]
    save_password: Option<bool>,
    #[serde(rename = "@ver")]
    version: Option<String>,
    #[serde(rename = "@userid")]
    user_id: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@password")]
    password: Option<String>,
    #[serde(rename = "@balance")]
    balance: Option<u32>,
    #[serde(rename = "@hintQ")]
    hint_question: Option<String>,
    #[serde(rename = "@hintA")]
    hint_answer: Option<String>,
    // Sub-tags:
    music: Option<VolumeSetting>,
    soundfx: Option<VolumeSetting>,
    zones: Option<Zones>,
    #[serde(rename = "bitty-crib")]
    bitty_crib: Option<BittyCrib>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct VolumeSetting {
    #[serde(rename = "@volume")]
    volume: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Zones {
    zone: Vec<Zone>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "zone")]
#[skip_serializing_none]
struct Zone {
    #[serde(rename = "@id")]
    id: Option<String>,
    gem: Option<Gem>,
    map: Option<Map>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Gem {
    #[serde(rename = "@count")]
    count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Map {
    #[serde(rename = "@tiles")]
    tiles: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct BittyCrib {
    screens: Option<Screens>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Screens {
    screen: Vec<Screen>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Screen {
    #[serde(rename = "@rid")]
    room_id: Option<String>,
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@name")]
    name: Option<String>,
    layout: Option<Layout>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Layout {
    walls: Option<Walls>,
    floor: Vec<Floor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Walls {
    wall: Vec<Wall>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Wall {
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@fill")]
    fill: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Floor {
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@type")]
    r#type: Option<u32>,
    #[serde(rename = "@fill")]
    fill: Option<String>,
}

fn main() -> io::Result<()> {
    let mut file = File::open("profile.rdf")?;
    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    let decoded_xml = decode_rdf(&encoded_data).unwrap();
    let deserialized_struct = from_str::<Profile>(&decoded_xml).unwrap();
    println!("{:#?}", deserialized_struct);

    Ok(())
}
