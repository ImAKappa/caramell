use chordparser::{
    pitch::{Note, Accidental, PitchClass},
    chords::{Chord, ChordQuality},
};

fn main() {
    println!("Hello World!");
    let my_note = Note { pitch: PitchClass::C, accidental: Some(Accidental::Sharp) };
    println!("{:?}", my_note.to_string());
}