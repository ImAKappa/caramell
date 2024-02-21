# caramell

Caramell is a simple lead sheet generator, loosely based on the [chordpro](https://www.chordpro.org/chordpro/chordpro-introduction/) specification.

See [https://playingchordcharts.com/chords#notation](https://playingchordcharts.com/chords#notation) for more details on chord notation

# (Proposed) Features

Input:

* Chords
    - [ ] Major
    - [ ] Minor
    - [ ] Sharps (`#`), Flats (`b`)
    - [ ] Numbers (7th, 9th, etc. caramell won't check if the chords make sense)
* Metadata
    - [ ] title
    - [ ] subtitle
    - [ ] artist
    - [ ] composer
    - [ ] year
    - [ ] key 
    - [ ] tempo
    - [ ] time signature
    - [ ] duration
    - [ ] capo
* Environments
    - [ ] start_of_chorus (short: `soc`)
    - [ ] end_of_chorus (short: `eoc`)

Output:

- [ ] Plaintext
- [ ] PDF

## Dev

Inspiration:

* [(Github)](https://github.com/ChordPro/chordpro)
* [(GitHub) joseluiscd/chordpro-rs](https://github.com/joseluiscd/chordpro-rs)

## Theory

[Music Transposition in Musescore](https://davidbolton.info/articles/interval-transposition-in-musescore/)

[Gallery of Interesting Music Notation](https://homes.luddy.indiana.edu/donbyrd/InterestingMusicNotation.html)

[Algorithm for Transposing Chords](https://music.stackexchange.com/questions/40041/algorithm-for-transposing-chords-between-keys)

[MuseScore Tonal Pitch Class](https://musescore.org/en/handbook/developers-handbook/plugin-development/tonal-pitch-class-enum)

[Line of Fifths](https://music.stackexchange.com/questions/17318/general-procedure-for-determining-the-name-of-an-interval-given-a-major-key-di/17365#17365)