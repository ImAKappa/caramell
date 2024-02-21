# ROADMAP

## Phase 1: Basics

### Stage A: Lexer & Parser for Common Chords

Goals:

- [X] Create a basic lexer for lyrics and chords
- [X] Create a basic parser for lyrics and chords
- [X] Parse the following chords
  - [X] Major
  - [X] Minor
  - [X] Chords with accidentals (# or b)
  - [X] 5ths (Power chords)
  - [X] 6ths
  - [X] Major 7th
  - [X] Minor 7th
  - [X] Augmented
  - [X] Diminished
  - [X] Suspended
  - [X] Major-Minor 7th (Dominant) 
  - [X] Minor-Major 7th
  - [X] Minor-Minor 7th (Diminished)
  - [X] Slash chords

Completed Feb 21, 2024

### Stage B: Plain-Text Output

- [ ] Output a plain-text lyric text file that is formatted with the chords above the corresponding in the lyrics

### Stage C: First Release on GitHub

Goals:

- [ ] Release as a command line tool on GitHub

## Phase 2: User-Friendly Edition

### Stage A: Typst Output

Goals: 

- [ ] Output Typst file
- [ ] Render Typst file as PDF

### Stage B: Nicer CLI

Goals:

- [ ] Switch to `clap`
- [ ] Print more helpful error messages

### Stage C: Second Release on GitHub

Goals:

- [ ] Second release as command line tool on GitHub
- [ ] Setup automated testing for Continuous Integration

## Phase 3: Internal Upgrades

### Stage A: Upgraded Lexer & Parser

Goals:

- [ ] More flexible lexer and parser
- [ ] Better error-handling and debug capabilities for lexer and parser

### Stage B: Keywords & Metadata

Goals:

- [ ] Handle metadata
  - [ ] Title
  - [ ] Lyricist
  - [ ] Composer
  - [ ] Arranger
- [ ] Handle keywords
  - [ ] Key
  - [ ] Intro
  - [ ] Verse/Refrain
  - [ ] Chorus
  - [ ] Repetitions
  - [ ] Outro

### Stage C: More chords

Goals:

- [ ] Handle the following additional chords
  - [ ] Half-Diminished chords
  - [ ] Alterations (e.g. `C7{b5}`, or other alterations like `b11` or `#9`)
  - [ ] Voicings (e.g. `C7{6,9}`)
  - [ ] Additional accidentals
    - [ ] Natural
    - [ ] Double sharp (`x`)
    - [ ] Double flat (`bb`)
  - [ ] Quartal chords

### Stage D: Third release on GitHub

Goals:

- [ ] Third release as command line tool on GitHub

## Phase 4: Transposition

### Stage A: Additional internals to support transposition

Goals:

- [ ] Automatic transposition between 12 major and 12 minor keys
- [ ] Option to specify transposition by semitones
- [ ] Option to specify transposition by new key

### Stage B: Fourth release on GitHub

Goals:

- [ ] Fourth release as command line tool on GitHub


## Phase 5: Collaboration

### Stage A: Configure GitHub repo for collaboration

Goals:

- [ ] Manage repo access
- [ ] Manage pull request permissions

### Stage B: Get contributors

- [ ] Create a `Contributing.md` guide
- [ ] Promote repo

## Phase 6: To Infinity and Beyond

Goals:

- [ ] Decide whether to take the project even further, create complimentary products, remaking the project, or simply moving on...
  - [ ] Custom GUI/text/WYSIWYG editor? Desktop vs web app?
  - [ ] Try to make money out of it?
  - [ ] Piano/TAB diagrams?
  - [ ] Multiple language support?
  - [ ] Rust library?