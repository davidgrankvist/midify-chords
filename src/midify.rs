use std::{env, fs};
use crate::models::*;
use crate::midify::util::*;

mod util;

const MIDDLE_C: u8 = 0x3c;
const QUARTER_DELTA: u8 = 0x19;

pub fn output_midi(song: &Song, out_file: &str) {
    let test_mode = env::var("TEST_MODE").is_ok();
    let song = if test_mode {
       get_test_midi_file()
    } else {
        song.midify()
    };

    println!("Writing MIDI to {}", out_file);
    fs::write(out_file, &song).expect(format!("Failed to write to {}", out_file).as_str());
}

impl Song {
    fn midify(&self) -> Vec<u8> {
        let chords: Vec<u8> = self.bars.iter()
            .map(| bar | {
                &bar.chords
            }).flatten().enumerate()
        .map(| (i, chord) | {
            chord.midify(i)
        }).flatten().collect();
        vec![
            Song::get_midi_header(),
            self.get_midi_track_preamble(chords.len()),
            chords,
            Song::get_midi_track_end()
        ].concat()
    }
    fn get_midi_header() -> Vec<u8> {
        vec![
            // MThd
            0x4d, 0x54, 0x68, 0x64,
            0x00, 0x00, 0x00, 0x06,
            0x00, 0x00,
            0x00, 0x01,
            0x01, 0xe0,
        ]
    }
    fn get_midi_track_preamble(&self, chord_chunks: usize) -> Vec<u8> {
        let chunks = chord_chunks + Self::get_midi_track_end().len();
        let chunks = to_bytes(chunks);
        vec![
            vec![
                // MTrk
                0x4d, 0x54, 0x72, 0x6b,
            ],
            chunks
        ].concat()
    }
    fn get_midi_track_end() -> Vec<u8> {
        vec![
            0x01, 0xff, 0x2f, 0x00,
        ]
    }
}

impl Chord {
    fn midify(&self, index: usize) -> Vec<u8> {
        let notes = self.to_notes();
        let delta = if index > 0 {
            QUARTER_DELTA
        } else {
            0x00
        };
        vec![
            // note on
            delta,
            0x90, notes[0], 0x50,
            0x00, notes[1], 0x50,
            0x00, notes[2], 0x50,

            // note off
            0x83, 0x47, notes[0], 0x00,
            0x00,       notes[1], 0x00,
            0x00,       notes[2], 0x00,
        ]
    }
    fn to_notes(&self) -> Vec<u8> {
        let root = *(&self.root.midify());
        let (third, fifth) = match &self.quality {
            Quality::Major => (4, 7),
            Quality::Minor => (3, 7),
            Quality::Diminished => (3, 6),
            Quality::Augmented => (4, 8),
            Quality::Suspended => (5, 7),

        };
        vec![root, root + third, root + fifth]
    }

}

impl Note {
    fn midify(&self) -> u8 {
        let letter = match &self.0 {
            Letter::C => MIDDLE_C,
            Letter::D => MIDDLE_C + 2,
            Letter::E => MIDDLE_C + 4,
            Letter::F => MIDDLE_C + 5,
            Letter::G => MIDDLE_C + 7,
            Letter::A => MIDDLE_C + 9,
            Letter::B => MIDDLE_C + 11,
        };
        match &self.1 {
            Some(Semitone::Sharp) => letter + 1,
            Some(Semitone::Flat) => letter - 1,
            None => letter,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_song_midify() {
        let song = Song {
            config: SongConfig {
                tempo: 120,
                time: TimeSignature {
                    numerator: NoteDuration::Quarter(4),
                    denominator: NoteDuration::Quarter(4),
                }
            },
            bars: vec![
                Bar {
                    chords: vec![
                        Chord {
                            duration: NoteDuration::Quarter(1),
                            root: Note(Letter::C, None),
                            quality: Quality::Major,
                        },
                        Chord {
                            duration: NoteDuration::Quarter(1),
                            root: Note(Letter::C, None),
                            quality: Quality::Major,
                        }
                    ]
                }
            ]
        };
        assert_eq!(song.midify(), get_test_midi_file());
    }
}

fn get_test_midi_file() -> Vec<u8> {
    vec![
        // MThd
        0x4d, 0x54, 0x68, 0x64,
        0x00, 0x00, 0x00, 0x06,
        0x00, 0x00,
        0x00, 0x01,
        0x01, 0xe0,

        // MTrk
        0x4d, 0x54, 0x72, 0x6b,
        0x00, 0x00, 0x00, 0x2c,

        // note on C, E, G
        0x00, 0x90, 0x3c, 0x50,
        0x00,       0x40, 0x50,
        0x00,       0x43, 0x50,
        // note off C, E, G
        0x83, 0x47, 0x3c, 0x00,
        0x00,       0x40, 0x00,
        0x00,       0x43, 0x00,

        // note on C, E, G
        0x19, 0x90, 0x3c, 0x50,
        0x00,       0x40, 0x50,
        0x00,       0x43, 0x50,
        // note off C, E, G
        0x83, 0x47, 0x3c, 0x00,
        0x00,       0x40, 0x00,
        0x00,       0x43, 0x00,

        0x01, 0xff, 0x2f, 0x00,
    ]
}
