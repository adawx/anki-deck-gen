use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use std::env;
use genanki_rs::{basic_model, Deck, Error, Note};
use serde::{Deserialize, Serialize};

// Take an argument for json File
// Read in json File
// pass metadata to create deck fn
//  create model and then deck
// Map entries to Cards in anki deck
// output deck

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

fn main() -> Result<(), Error> {
    println!("{:#?}", read_in_deck("data/canto_deck.json").unwrap());

    //Example from genanki_rs
    let mut deck = Deck::new(1234, "Example Deck", "Example Deck containing 2 Flashcards");
    deck.add_note(Note::new(
        basic_model(),
        vec!["What is the capital of France?", "Paris"],
    )?);
    deck.add_note(Note::new(
        basic_model(),
        vec!["What is the capital of Germany?", "Berlin"],
    )?);
    deck.write_to_file("output/output.apkg")?;

    Ok(())
}

fn read_in_deck<P: AsRef<Path>>(path: P) -> std::io::Result<JsonDeck> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);

    let json: JsonDeck = serde_json::from_reader(reader).expect("JSON was not well-formatted");

    Ok(json)
}
