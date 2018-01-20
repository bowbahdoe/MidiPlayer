extern crate bit_vec;

mod model;

use std::fs::File;
use model::note::{Note, Of, Accidental};
use model::midi_parser::{MidiParser};
use model::chunk_parsing::parse_initial_header;

fn main() {
    // println!("{:?}", Accidental::Sharp as u8);
    // let n = Note::new(Of::A, Accidental::Sharp, 3);
    // println!("{:?}", n);
    // println!("{:?}", n.to_int());

    if let Ok(file) = File::open("piano.mid") {
        if let Some(parser) = MidiParser::new(file) {
            for track in parser.track_chunks() {
                println!("{}", track);
            }
            println!();
            for track in parser.header_chunks() {
                println!("{:?}", parse_initial_header(track));
            }
            println!();
            for track in parser.chunks() {
                println!("{:?}", track);
            }
        }
    }
}
