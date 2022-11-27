# anki-deck-gen

Tool to build Anki Decks from JSON files using Rust. Using [genanki-rs](https://github.com/yannickfunk/genanki-rs). Not affiliated with the main anki project.

Mostly built for me to learn some rust, but also to help me be less lazy in my cantonese.

### Build tips

Cargo.

Running the binary with commands from build

`target/debug/anki-deck-gen --input data/canto_deck.json --output output/test.apkg`


### MVP Features:
- CL arguments (--file --output)
- Custom CSS for anki cards
- Optional secondary main text
- Error handling
- Argument handling
- force output file to be .apkg
- Optional secondary back text?

