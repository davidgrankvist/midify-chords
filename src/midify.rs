use std::fs;
use crate::models::*;

const MIDDLE_C: u8 = 0x3c;
const QUARTER_DELTA: u8 = 0x19;

pub fn output_midi(song: &Song, out_file: &str) {
    println!("Writing MIDI to {}", out_file);

    let song = song.midify();
    fs::write(out_file, &song).expect(format!("Failed to write to {}", out_file).as_str());
}

fn get_dummy_track() -> Vec<u8> {
    vec![
        // MThd
        0x4d, 0x54, 0x68, 0x64,
        0x00, 0x00, 0x00, 0x06,
        0x00, 0x00,
        0x00, 0x01,
        0x01, 0xe0,

        // MTrk
        0x4d, 0x54, 0x72, 0x6b,
        0x00, 0x00, 0x00, 0x2e,

        // note on C, E, G
        0x80, 0x00, 0x90, 0x3c, 0x50,
        0x00,       0x40, 0x50,
        0x00,       0x43, 0x50,
        // note off C, E, G
        0x83, 0x47, 0x3c, 0x00,
        0x00,       0x40, 0x00,
        0x00,       0x43, 0x00,

        // note on C, E, G
        0x80, 0x19, 0x90, 0x3c, 0x50,
        0x00,       0x40, 0x50,
        0x00,       0x43, 0x50,
        // note off C, E, G
        0x83, 0x47, 0x3c, 0x00,
        0x00,       0x40, 0x00,
        0x00,       0x43, 0x00,

        0x01, 0xff, 0x2f, 0x00,
    ]
}

fn to_midi(n: &Note) -> u8 {
    let letter = match &n.0 {
        Letter::C => MIDDLE_C,
        Letter::D => MIDDLE_C + 2,
        Letter::E => MIDDLE_C + 4,
        Letter::F => MIDDLE_C + 6,
        Letter::G => MIDDLE_C + 8,
        Letter::A => MIDDLE_C + 10,
        Letter::B => MIDDLE_C + 12,
    };
    match &n.1 {
        Some(Semitone::Sharp) => letter + 1,
        Some(Semitone::Flat) => letter - 1,
        None => letter,
    }
}

trait Midi {
    fn midify(&self) -> Vec<u8>;
}

fn to_midi_delta(index: usize) -> Vec<u8> {
    let index: u16 = index.try_into().unwrap();
    let delta: u16 = QUARTER_DELTA.try_into().unwrap();
    let delta = index * delta;

    let upper = 0x80 | ( delta >> 8 );
    let lower = 0x00ff & delta;
    vec![upper.try_into().unwrap(), lower.try_into().unwrap()]
}

fn to_bytes(num: usize) -> Vec<u8> {
    let num: u32 = num.try_into().unwrap();

    let upper = num >> 24;
    let upper_mid = 0xff & (num >> 16);
    let lower_mid = 0xff & (num >> 8);
    let lower = 0xff & num;

    let upper: u8 = upper.try_into().unwrap();
    let upper_mid: u8 = upper_mid.try_into().unwrap();
    let lower_mid: u8 = lower_mid.try_into().unwrap();
    let lower: u8 = lower.try_into().unwrap();
    vec![upper, upper_mid, lower_mid, lower]
}

impl Chord {
    fn to_notes(&self) -> Vec<u8> {
        let root = to_midi(&self.root);
        // assume major chord for now
        let third = root + 4;
        let fifth = root + 7;
        vec![root, third, fifth]
    }

    fn midify(&self, index: usize) -> Vec<u8> {
        let notes = self.to_notes();
        let delta = to_midi_delta(index);
        vec![
            delta,
            vec![
                // note on
                0x90,       notes[0], 0x50,
                0x00,       notes[1], 0x50,
                0x00,       notes[2], 0x50,

                // note off
                0x83, 0x47, notes[0], 0x00,
                0x00,       notes[1], 0x00,
                0x00,       notes[2], 0x00,
            ]
        ].concat()
    }
}

impl Midi for Song {
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
}

impl Song {
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
        assert_eq!(song.midify(), get_dummy_track());
    }

    #[test]
    fn test_to_midi_delta() {
        assert_eq!(to_midi_delta(0), vec![0x80, 0x00]);
        assert_eq!(to_midi_delta(1), vec![0x80, QUARTER_DELTA]);
        assert_eq!(to_midi_delta(100), vec![0x89, 0xc4]);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(0xaabbccdd), vec![0xaa, 0xbb, 0xcc, 0xdd])
    }
}
