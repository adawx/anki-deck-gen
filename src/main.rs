use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct NoteInfo {
    front_text: String,
    romanisation: String,
    back_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonDeck {
    name: String,
    tts: bool,
    api_key: String,
    entries: Vec<NoteInfo>,
}

fn main() {
    println!("{:#?}", read_in_deck("data/canto_deck.json").unwrap());
}

fn read_in_deck<P: AsRef<Path>>(path: P) -> std::io::Result<JsonDeck> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);

    let json: JsonDeck = serde_json::from_reader(reader).expect("JSON was not well-formatted");

    Ok(json)
}
