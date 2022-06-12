use std::fs;
use crate::models::*;

pub fn output_midi(song: &Song, out_file: &str) {
    println!("Writing MIDI to {}", out_file);
    fs::write(out_file, "wait what this is not midi").expect(format!("Failed to write to {}", out_file).as_str());
}
