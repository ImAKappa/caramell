pub mod chords;
pub mod lexer;
/// Module for parsing chords
pub mod pitch;
pub use logos;
pub mod parser;

const HALF_STEP: isize = 1;
const WHOLE_STEP: isize = HALF_STEP * 2;

pub fn print(song: String) {
    match parser::parse(song) {
        Ok(phrases) => {
            for p in phrases {
                println!("{:?}", p);
            }
        }
        Err(err) => eprintln!("error: {err}"),
    }
}
