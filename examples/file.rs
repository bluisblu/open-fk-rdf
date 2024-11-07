use std::{fs::File, io::{self, Read}};
use open_fk_rdf::decode_rdf;

fn main() -> io::Result<()> {
    let mut file = File::open("profile.rdf")?;
    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    let decoded_xml = decode_rdf(&encoded_data);
    println!("Decoded XML: {}", decoded_xml);

    Ok(())
}