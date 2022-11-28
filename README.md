# anki-deck-gen

Tool to build Anki Decks from JSON files using Rust. The aim is to be able to using a source controllable file like json that is easily readable and editable to maintain Anki decks, and update, edit and built relatively easily without having to deal with maintaining the deck in Anki. Initially this project is under the guise of learning vocabulary for languages (but may be extended to be more generic in future if I get that far).

The ultimate goal is to be able to provide a deck of notes in json format, some custom css to style cards, and optionally an API key to a TTS service or provide audio files to have audio support.

Using [genanki-rs](https://github.com/yannickfunk/genanki-rs). Not affiliated with the main anki project.

Mostly built for me to learn some rust, but also to help me be less lazy in my cantonese. Example JSON file structure can be found in `data/`. Will publish a binary once I figure out wtf is going on. 


### Build tips

Built using Cargo.

`cargo build`

Running the binary with commands from build

`target/debug/anki-deck-gen --input data/canto_deck.json --output output/test.apkg`


### V1 Features:
- Optional secondary main text
- Error handling
- Argument handling
- Optional secondary back text?
- Published binary & crate

### Roadmap
#### V1.1:
- Custom CSS for anki cards
- force output file to be .apkg

#### V1.2:
- TTS support
  - API tts
  - Pre-provided files
