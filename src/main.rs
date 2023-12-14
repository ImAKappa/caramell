use chordparser::{
    pitch::{Note, Accidental, PitchClass},
    chords::{Chord, ChordQuality},
    lexer::{Token},
    logos::Logos,
};

fn main() {
    println!("Hello World!");
    let my_note = Note { pitch: PitchClass::C, accidental: Some(Accidental::Sharp) };
    println!("{:?}", my_note.to_string());

    let mut lex = Token::lexer("Never gonna [BbM7]give you up");

    // while let token = lex.next() {
    //     println!("[{:?}] {:?} {:?}", token, token.span(), token.slice())
    // }
    // for token in lex {
    // }
}