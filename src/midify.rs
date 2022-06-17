use std::fs;
use crate::models::*;

const MIDDLE_C: u8 = 0x3c;

pub fn output_midi(song: &Song, out_file: &str) {
    println!("Writing MIDI to {}", out_file);

    let song = get_dummy_track();
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
        0x00, 0x00, 0x00, 0x18,

        // note on C, E, G
        0x00, 0x90, 0x3c, 0x50,
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

impl Chord {
    fn to_notes(&self) -> Vec<u8> {
        let root = to_midi(&self.root);
        // assume major chord for now
        let third = root + 4;
        let fifth = root + 7;
        vec![root, third, fifth]
    }
}

impl Midi for Chord {
    fn midify(&self) -> Vec<u8> {
        let notes = self.to_notes();
        // assume quarter notes for now
        vec![
            // note on
            0x00, 0x90, notes[0], 0x50,
            0x00,       notes[1], 0x50,
            0x00,       notes[2], 0x50,

            // note off
            0x83, 0x47, notes[0], 0x00,
            0x00,       notes[1], 0x00,
            0x00,       notes[2], 0x00,
        ]
    }
}

impl Midi for Bar {
    fn midify(&self) -> Vec<u8> {
        let chords: Vec<Vec<u8>> = self.chords.iter()
            .map(| chord | {
                chord.midify()
            }).collect();
        chords.concat()
    }
}

impl Midi for Song {
    fn midify(&self) -> Vec<u8> {
        let bars: Vec<u8> = self.bars.iter()
            .map(| bar | {
                bar.midify()
            }).collect::<Vec<Vec<u8>>>().concat();
        vec![
            Song::get_midi_header(),
            self.get_midi_track_preamble(),
            bars,
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
    fn get_midi_track_preamble(&self) -> Vec<u8> {
        vec![
            // MTrk
            0x4d, 0x54, 0x72, 0x6b,
            0x00, 0x00, 0x00, 0x18,
        ]
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
                        }
                    ]
                }
            ]
        };
        assert_eq!(song.midify(), get_dummy_track());
    }
}
