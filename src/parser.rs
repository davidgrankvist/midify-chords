use crate::models::*;
use regex::*;

const CHORD_PATTERN: &str = r"^([A-G])(#|b)?(m|dim|aug)?$";

pub fn parse_song(raw_song: &str, song_config: SongConfig) -> Song {
    println!("Parsing this song:\n{}", &raw_song);

    let re = Regex::new(CHORD_PATTERN).unwrap();

    let bars: Vec<Bar> = raw_song.split("|")
        .map(| bar | {
            let chords: Vec<Chord> = bar.split_whitespace()
                .map(| chord |{
                    let chord = re.captures_iter(chord).next().expect("Chord did not match pattern");
                    let letter = chord.get(1).expect("Missing chord root letter")
                        .as_str().chars().next().unwrap().into();
                    let semitone: Option<Semitone> = match chord.get(2) {
                        Some(m) => {
                            let c = m.as_str().chars().next().unwrap();
                            Some(c.into())
                        },
                        _ => None
                    };
                    let quality: Quality = match chord.get(3) {
                        Some(q) => {
                            q.as_str().into()
                        },
                        _ => Quality::Major
                    };
                    let root = Note(letter, semitone);
                    Chord {
                        duration: 1,
                        root,
                        quality,
                    }
                }).collect();
            if chords.len() > song_config.time.numerator.into() {
                panic!("This bar bar does not match the {}/{} time signature:\n{}",
                       &song_config.time.numerator, &song_config.time.denominator, &bar);
            }
            Bar {
                chords,
            }
        }).collect();
    let song = Song {
        config: song_config,
        bars,
    };
    dbg!(&song);
    song
}

impl From<char> for Letter {
    fn from(c: char) -> Self {
        match c {
            'A' => Letter::A,
            'B' => Letter::B,
            'C' => Letter::C,
            'D' => Letter::D,
            'E' => Letter::E,
            'F' => Letter::F,
            'G' => Letter::G,
            _ => panic!("Letter {} is not a valid note (A-G)", c)
        }
    }
}

impl From<char> for Semitone {
    fn from(c: char) -> Self {
        match c {
            '#' => Semitone::Sharp,
            'b' => Semitone::Flat,
            _ => panic!("Letter {} is not a valid semitone (# or b)", c)
        }
    }
}

impl From<&str> for Quality {
    fn from(s: &str) -> Self {
        match s {
            "m" => Quality::Minor,
            "dim" => Quality::Diminished,
            "aug" => Quality::Augmented,
            _ => panic!("String {} is not a valid chord quality (m, dim or aug)", s)
        }
    }
}
