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
    menu: Option<Menu>,
    games: Option<Games>,
    trophies: Option<Trophies>,
    milestones: Option<Milestones>,
    zones: Option<Zones>,
    #[serde(rename = "bitty-crib")]
    bitty_crib: Option<BittyCrib>,
    statistics: Option<Statistics>,
    trunk: Option<Trunk>,
    dirt: Option<Dirt>,
    distractions: Option<Distractions>,
    chats: Option<Chats>,
    artefacts: Option<Artefacts>,
    quests: Option<Quests>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct VolumeSetting {
    #[serde(rename = "@volume")]
    volume: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Menu {
    items: Option<Items>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Items {
    #[serde(rename = "@fixed")]
    fixed: Option<bool>,
    item: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Item {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@total")]
    total: Option<u32>,
    #[serde(rename = "@used")]
    used: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Games {}

#[derive(Debug, Serialize, Deserialize)]
struct Trophies {}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Milestones {
    milestone: Option<Vec<Milestone>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Milestone {
    #[serde(rename = "@id")]
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Zones {
    zone: Option<Vec<Zone>>,
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
struct Statistics {
    #[serde(rename = "@date")]
    date: Option<String>,
    #[serde(rename = "@minutes")]
    minutes: Option<u32>,
    #[serde(rename = "@days")]
    days: Option<u32>,
    #[serde(rename = "@sessions")]
    sessions: Option<u32>,
    funkeys: Option<Funkeys>,
    seriess: Option<Seriess>,
    crib: Option<Crib>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Funkeys {
    funkey: Option<Vec<Funkey>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Funkey {
    #[serde(rename = "@t")]
    timestamp: Option<String>,
    #[serde(rename = "@count")]
    count: Option<u32>,
    #[serde(rename = "@id")]
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Seriess {
    series: Option<Vec<Series>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Series {
    #[serde(rename = "@id")]
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Crib {
    #[serde(rename = "@uploaded")]
    uploaded: Option<u32>,
    #[serde(rename = "@server")]
    server: Option<u32>,
    #[serde(rename = "@localy")]
    localy: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Trunk {
    items: Option<Items>,
    familiars: Option<Familiars>,
    moods: Option<Moods>,
    jammers: Option<Jammers>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Familiars {}

#[derive(Debug, Serialize, Deserialize)]
struct Moods {}

#[derive(Debug, Serialize, Deserialize)]
struct Jammers {}

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct BittyCrib {
    screens: Option<Screens>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Screens {
    screen: Option<Vec<Screen>>,
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
    floor: Option<Vec<Floor>>,
    trim: Option<Trim>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Walls {
    wall: Option<Vec<Wall>>,
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

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
struct Trim {
    #[serde(rename = "@fill")]
    fill: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dirt {
    #[serde(rename = "@timer")]
    timer: Option<u32>,
    #[serde(rename = "@level")]
    level: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Distractions {}

#[derive(Debug, Serialize, Deserialize)]
struct Chats {}

#[derive(Debug, Serialize, Deserialize)]
struct Artefacts {}

#[derive(Debug, Serialize, Deserialize)]
struct Quests {}

fn main() -> io::Result<()> {
    let mut file = File::open("profile.rdf")?;
    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    let decoded_xml = decode_rdf(&encoded_data).unwrap();
    let deserialized_struct = from_str::<Profile>(&decoded_xml).unwrap();
    println!("{:#?}", deserialized_struct);

    Ok(())
}
