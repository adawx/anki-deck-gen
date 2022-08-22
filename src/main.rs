use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use serde_json;

//Todo List
//Figure out file structure for audio files
//Api call & saving audio to disk
//reading audio file in & adding to deck
//save to deck
//do for all entries?

fn main() {
    read_in_file().unwrap();
}

fn read_in_file() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    //TODO: (ad) If no arg provided the prompt user to input a apth

    println!("Making deck from file: {}", file_path);

    let json_file = File::open("data/deck.json")?;
    let mut buf_reader = BufReader::new(json_file);
    let mut contents = String::new();   
    buf_reader.read_to_string(&mut contents)?;

    let json: serde_json::Value =
    serde_json::from_str(&contents).expect("JSON was not well-formatted");

    //TODO: (ad) Check present fields in json, if missing fillable fields prompt user, else error if blank? maybe only need entries and prompt for rest??
    //Arguments: deck name, tts on, tts provider, tts key, ...


    println!("{:#?}", json);
    Ok(())
}
