/// Module for parsing chords
#[derive(Debug, PartialEq)]
enum Tone {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G
}

impl Tone {
    fn new(s: &str) -> Result<Self, String> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("unknown tone".to_string())
        }
    }
}

#[derive(Debug, PartialEq)]
enum ToneModifier {
    DoubleSharp,
    Sharp,
    Flat,
    Natural,
    DoubleFlat,
}

impl ToneModifier {
    fn new(s: &str) -> Result<Self, String> {
        match s {
            "x" => Ok(Self::DoubleSharp),
            "#" => Ok(Self::Sharp),
            "♮" => Ok(Self::Natural),
            "b" => Ok(Self::Flat),
            "bb" => Ok(Self::DoubleFlat),
            _ => Err("unknown tone modifier".to_string())
        }
    }
}

#[derive(Debug, PartialEq)]
struct Note {
    tone: Tone,
    modifier: Option<ToneModifier>,
}

impl Note {
    fn new(s: &str) -> Result<Self, String> {
        let tone = Tone::new(&s[0..1])?;
        let modifier = if s.len() > 1 {
                Some(ToneModifier::new(&s[1..])?)
        } else { None };
        Ok(Self { tone, modifier, })
    }
}

#[derive(Debug, PartialEq)]
enum ChordQuality {
    Diminished,
    Minor,
    Major,
    Augmented,
}

#[derive(Debug, PartialEq)]
struct Chord {
    root: Note,
    quality: ChordQuality,
}

impl Chord {
    fn new(s: &str) -> Result<Self, String> {
        Ok(Chord {
            root: Note { tone: Tone::C, modifier: None, },
            quality: ChordQuality::Major,
        })
    }
}

// Chord Degree (European: C - B, Roman: I - VII)

// Chord Quality: Major, Minor, 7ths, Aug

// Chord Inversions: Root, 1st, 2nd

// Transposition

#[cfg(test)]
mod tests {
    use super::*;

    // Tone
    #[test]
    fn parse_tone() {
        assert_eq!(Tone::new("C"), Ok(Tone::C));
        assert_eq!(Tone::new("@"), Err("unknown tone".to_string()));
    }
    // ToneModifier
    #[test]
    fn parse_tone_modifier() {
        assert_eq!(ToneModifier::new("♮"), Ok(ToneModifier::Natural));
        assert_eq!(ToneModifier::new("##"), Err("unknown tone modifier".to_string()));
    }
    // Note
    #[test]
    fn parse_note() {
        assert_eq!(Note::new("C#"), Ok(Note { tone: Tone::C, modifier: Some(ToneModifier::Sharp) }));
        assert_eq!(Note::new("G"), Ok(Note { tone: Tone::G, modifier: None}));
        assert_eq!(Note::new("$"), Err("unknown tone".to_string()));
        assert_eq!(Note::new("E%"), Err("unknown tone modifier".to_string()));
    }
    // Chord

    #[test]
    fn parse_root_chord() {
        assert_eq!(Chord::new("C"),
            Ok(
                Chord { 
                    root: Note { tone: Tone::C, modifier: None },
                    quality: ChordQuality::Major,
                }
            )
        );
    }
}

// mod tests {
//     use super::*;

//     #[test]
//     fn parse_major_chord() {

//     }

//     #[test]
//     fn parse_minor_chord() {

//     }

//     #[test]
//     fn parse_major_7th_chord() {

//     }

//     #[test]
//     fn parse_lyrics_and_chords() {

//     }

//     #[test]
//     fn parse_metadata_title() {

//     }
// }
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_major_chord() {

//     }

//     #[test]
//     fn parse_minor_chord() {

//     }

//     #[test]
//     fn parse_major_7th_chord() {

//     }

//     #[test]
//     fn parse_lyrics_and_chords() {

//     }

//     #[test]
//     fn parse_metadata_title() {

//     }
// }