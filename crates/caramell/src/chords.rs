use crate::pitch::{Note, PitchClass};

#[derive(Debug, PartialEq)]
pub enum ChordQuality {
    Diminished,
    Minor,
    Major,
    Augmented,
    Sus,
}

#[derive(Debug, PartialEq)]
pub struct Chord {
    pub root: Note,
    pub quality: ChordQuality,
}

impl Chord {
    pub fn new(s: &str) -> Result<Self, String> {
        Ok(Chord {
            root: Note { pitch: PitchClass::C, accidental: None, },
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
    use crate::pitch::{Note, PitchClass};

    #[test]
    fn parse_root_chord() {
        assert_eq!(Chord::new("C"),
            Ok(
                Chord { 
                    root: Note { pitch: PitchClass::C, accidental: None },
                    quality: ChordQuality::Major,
                }
            )
        );
    }
}