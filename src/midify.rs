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
