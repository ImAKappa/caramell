mod chords;
mod lexer;
mod parser;
mod pitch;
mod printing;

const HALF_STEP: isize = 1;
const WHOLE_STEP: isize = HALF_STEP * 2;

pub fn print(song: String) {
    match parser::parse(song) {
        Ok(lines) => {
            // lines.debug_print();
            let fmt_song = printing::fmt_lyrics_and_chords(lines);
            println!("{fmt_song}");
        }
        Err(err) => eprintln!("error: {err}"),
    }
}
