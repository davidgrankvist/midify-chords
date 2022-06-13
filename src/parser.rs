use crate::*;
use crate::models::*;

pub fn parse_song(raw_song: &str, config: &Config) -> Song {
    println!("Parsing this song:\n{}", &raw_song);
    let time = TimeSignature::new(config.time.0, config.time.1);

    let bars: Vec<Bar> = raw_song.split("|")
        .map(| bar | {
            let chords: Vec<Chord> = bar.split_whitespace()
                .map(| chord |{
                    let root = chord.chars().next().expect("Failed to parse chord");
                    Chord {
                        duration: 1,
                        root,
                    }
                }).collect();
            Bar {
                chords,
            }
        }).collect();
    let song = Song {
        tempo: config.tempo,
        time,
        bars,
    };
    dbg!(&song);
    song
}
