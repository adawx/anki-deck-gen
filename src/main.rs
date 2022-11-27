use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use genanki_rs::{Deck, Error, Field, Model, Note, Template};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct NoteInfo {
    front_text: String,
    front_text2: Option<String>,
    back_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonDeck {
    name: String,
    description: String,
    tts: bool,
    api_key: String,
    entries: Vec<NoteInfo>,
}

fn main() -> Result<(), Box<Error>> {
    let args = Args::parse();

    let json_deck = read_in_deck(args.input).unwrap();

    let mut rng = rand::thread_rng();

    let deck_id: usize = rng.gen_range(1..9999999999);
    let model_id: usize = rng.gen_range(1..9999999999);

    let mut deck = Deck::new(deck_id, &json_deck.name, &json_deck.description);

    let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
    let my_model = Model::new(
        model_id,
        "card model",
        vec![Field::new("Front Side"), Field::new("Back Side")],
        vec![Template::new("Card 1")
            .qfmt("{{Front Side}}")
            .afmt(r#"{{Front Side}}<hr id="answer">{{Back Side}}"#)],
    )
    .css(custom_css);

    for entry in json_deck.entries.iter() {
        let note = Note::new(my_model.clone(), vec![&entry.front_text, &entry.back_text])?;
        deck.add_note(note)
    }

    let path = String::from(args.output.to_string_lossy());
    deck.write_to_file(&path)?;

    Ok(())
}

fn read_in_deck<P: AsRef<Path>>(path: P) -> std::io::Result<JsonDeck> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);

    let json: JsonDeck = serde_json::from_reader(reader).expect("JSON was not well-formatted");

    Ok(json)
}

//fn map_deck(json: JsonDeck, model: Model, deck: Deck) {}
