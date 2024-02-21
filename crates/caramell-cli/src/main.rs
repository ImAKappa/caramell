use std::fs;
use caramell;

fn main() {
    let song = fs::read_to_string("../../examples/input/rickroll.cho")
        .expect("Unable to read file");

    caramell::print(song);

    let song = fs::read_to_string("../../examples/input/chords.cho")
        .expect("Unable to read file");

    caramell::print(song);
}