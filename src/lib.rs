use std::io::{self, Read};
use models::*;

use crate::parser::parse_song;
use crate::midify::output_midi;

pub mod models;
pub mod parser;
pub mod midify;

pub fn run(config: Config) {
    println!("Reading song from STDIN");
    let mut song = String::new();
    io::stdin().read_to_string(&mut song).expect("Failed to read song from STDIN");

    println!("Parsing song");
    let song = parse_song(&song, config.song_config);
    println!("Converting to MIDI");
    output_midi(&song, &config.out_file);
    println!("Done!");
}

pub struct Config {
    song_config: SongConfig,
    out_file: String,
}

impl Config {
    pub fn new(tempo: u16, out_file: String) -> Config {
        Config {
            song_config: SongConfig { 
                tempo,
                time: TimeSignature { numerator: NoteDuration::Quarter(4), denominator: NoteDuration::Quarter(4) },
            },
            out_file,
        }
    }
}
