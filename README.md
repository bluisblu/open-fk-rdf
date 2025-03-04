# open-fk-rdf

An encoding/decoding library for RDFv2 files, ported from [OpenFK](https://github.com/GittyMac/OpenFK/) into rust.

## Features

- Encode and decode xml data from the `.rdf` files from the game U.B. Funkeys
- Does not provide the serialization types specific to each file, you will have to implement them using something like [quick-xml](https://github.com/tafia/quick-xml)

## Usage

Just use it as a git crate:
```toml
[dependencies]
open-fk-rdf = { git = "https://github.com/bluisblu/open-fk-rdf" }
```

A quick and dirty example:
```rs
use std::{fs::File, io::{self, Read}};
use open_fk_rdf::decode_rdf;

fn main() -> io::Result<()> {
    let mut file = File::open("profile.rdf")?;
    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    let decoded_xml = decode_rdf(&encoded_data).unwrap();
    println!("Decoded XML: {}", decoded_xml);

    Ok(())
}
```
This should work with any file. See the examples folder for a quick-xml implementation.

## Why does this exist?

This library exists to preserve the functionality of a beloved childhood game, U.B. Funkeys. While I can't take credit for reverse-engineering the file format, this project aims to provide a foundation for working with the data by separating out the encoding logic.