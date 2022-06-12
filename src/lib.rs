use std::io::{self, Read};
use crate::parser::parse_song;
use crate::midify::output_midi;

pub mod models;
pub mod parser;
pub mod midify;

pub fn run(config: Config) {
    println!("Reading song from STDIN");
    let mut song = String::new();
    io::stdin().read_to_string(&mut song).expect("Failed to read song from STDIN");

    let song = parse_song(&song, &config);
    output_midi(&song, &config.out_file);
}

pub struct Config {
    tempo: u16,
    time: (u8, u8),
    out_file: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            tempo: 120,
            time: (4, 4),
            out_file: String::from("test.mid")
        }
    }
}
