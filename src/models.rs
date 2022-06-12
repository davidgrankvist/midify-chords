pub struct Song {
    pub tempo: u16,
    pub time: TimeSignature,
    pub bars: Vec<Bar>
}

pub struct TimeSignature(u8, u8);

impl TimeSignature {
    pub fn new(numerator: u8, denominator: u8) -> TimeSignature {
        TimeSignature(numerator, denominator)
    }
}

pub struct Bar {
    chords: Vec<Chord>
}

pub struct Chord {
    duration: u8,
    root: char,
}
