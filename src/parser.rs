use crate::*;
use crate::models::*;

pub fn parse_song(raw_song: &str, config: &Config) -> Song {
    println!("Parsing this song:\n{}", &raw_song);
    Song {
        tempo: config.tempo,
        time: TimeSignature::new(config.time.0, config.time.1),
        bars: vec![]
    }
}
