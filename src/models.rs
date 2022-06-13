#[derive(Debug)]
pub struct Song {
    pub config: SongConfig,
    pub bars: Vec<Bar>
}

#[derive(Debug)]
pub struct SongConfig {
    pub tempo: u16,
    pub time: TimeSignature,
}

#[derive(Debug)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

#[derive(Debug)]
pub struct Bar {
    pub chords: Vec<Chord>
}

#[derive(Debug)]
pub struct Chord {
    pub duration: u8,
    pub root: Note,
    pub quality: Quality,
}

#[derive(Debug)]
pub struct Note(pub Letter, pub Option<Semitone>);

#[derive(Debug)]
pub enum Letter {
    A, B, C, D, E, F, G
}

#[derive(Debug)]
pub enum Semitone {
    Sharp, Flat
}

#[derive(Debug)]
pub enum Quality {
    Major, Minor, Diminished, Augmented
}
