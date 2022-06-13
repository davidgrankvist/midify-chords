#[derive(Debug)]
pub struct Song {
    pub tempo: u16,
    pub time: TimeSignature,
    pub bars: Vec<Bar>
}

#[derive(Debug)]
pub struct TimeSignature(u8, u8);

impl TimeSignature {
    pub fn new(numerator: u8, denominator: u8) -> TimeSignature {
        TimeSignature(numerator, denominator)
    }
}

#[derive(Debug)]
pub struct Bar {
    pub chords: Vec<Chord>
}

#[derive(Debug)]
pub struct Chord {
    pub duration: u8,
    pub root: char,
}
