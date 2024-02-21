use crate::{HALF_STEP, WHOLE_STEP};
use std::fmt;

/// Pitch-related functionality

/// Pitch Space is the set of all pitches: A, B, C, D, E, F, G
/// `caramell` follows the Western 12-tone system, with 7 distinct pitch classes
const PITCH_SPACE_SIZE: usize = 7;

const PITCH_POSITION_OFFSET: usize = 14;
const LINE_OF_FIFTHS: &[&str] = &[
    "Fbb", "Cbb", "Gbb", "Dbb", "Abb", "Ebb", "Bbb", "Fb", "Cb", "Gb", "Db", "Ab", "Eb", "Bb", "F",
    "C", "G", "D", "A", "E", "B", "F#", "C#", "G#", "D#", "A#", "E#", "B#", "F##", "C##", "G##",
    "D##", "A##", "E##", "B##",
];

/// PitchClass according to 7-tone pitch space
/// The order of the variants is important;
/// each subsequent variant a Perfect 5th (P5) above the previous one
/// For example, the interval from F to C is a P5
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PitchClass {
    F,
    C,
    G,
    D,
    A,
    E,
    B,
}

/// Accidentals modify a pitch
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Accidental {
    // Raises an unaltered pitch a whole step, or a sharped pitch one half step
    DoubleSharp = WHOLE_STEP,
    // Raises a pitch a half step
    Sharp = HALF_STEP,
    // Cancels a previous accidental, or indicates an unaltered pitch
    Natural = 0,
    // Lowers a pitch a half step
    Flat = -HALF_STEP,
    // Lowers a pitch an unaltered pitch a whole step, or a flatted pitch one half step
    DoubleFlat = -WHOLE_STEP,
}

impl PitchClass {
    pub fn new(s: &str) -> Result<Self, String> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("unknown pitch".to_string()),
        }
    }

    pub fn from_int(i: isize) -> Result<Self, String> {
        match i {
            0 => Ok(Self::F),
            1 => Ok(Self::C),
            2 => Ok(Self::G),
            3 => Ok(Self::D),
            4 => Ok(Self::A),
            5 => Ok(Self::E),
            6 => Ok(Self::B),
            _ => Err(format!(
                "expected int 0..6, cannot map int '{i}' to PitchClass"
            )),
        }
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Accidental {
    pub fn new(s: &str) -> Result<Self, String> {
        match s {
            "##" => Ok(Self::DoubleSharp),
            "#" => Ok(Self::Sharp),
            "♮" => Ok(Self::Natural),
            "b" => Ok(Self::Flat),
            "bb" => Ok(Self::DoubleFlat),
            _ => Err("unknown accidental".to_string()),
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub pitch: PitchClass,
    pub accidental: Option<Accidental>,
}

impl Note {
    pub fn new(s: &str) -> Result<Self, String> {
        let pitch = PitchClass::new(&s[0..1])?;
        let accidental = if s.len() > 1 {
            Some(Accidental::new(&s[1..])?)
        } else {
            None
        };
        Ok(Self { pitch, accidental })
    }

    /// Determines position of tonal pitch class along the line of fifths
    /// The "line of fifths" is a representation of pitch space where there are no enharmonic equivalents;
    /// Instead, pitches classes like F# and Gb are treated as distinct
    pub fn position(&self) -> i32 {
        let accidental = match &self.accidental {
            Some(a) => *a as i32,
            None => 0,
        };
        PITCH_SPACE_SIZE as i32 * accidental + self.pitch as i32
    }

    pub fn transpose(&self, half_steps: i32) -> Note {
        todo!("transpose method in progress");
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let accidental = match &self.accidental {
            Some(a) => a.to_string(),
            None => "".to_string(),
        };
        write!(f, "{}{}", &self.pitch.to_string(), accidental)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pitch_class() {
        assert_eq!(PitchClass::new("C"), Ok(PitchClass::C));
        assert_eq!(PitchClass::new("@"), Err("unknown pitch".to_string()));
    }

    #[test]
    fn parse_accidental() {
        assert_eq!(Accidental::new("♮"), Ok(Accidental::Natural));
        assert_eq!(Accidental::new("&"), Err("unknown accidental".to_string()));
    }

    #[test]
    fn parse_note() {
        assert_eq!(
            Note::new("C#"),
            Ok(Note {
                pitch: PitchClass::C,
                accidental: Some(Accidental::Sharp)
            })
        );
        assert_eq!(
            Note::new("G"),
            Ok(Note {
                pitch: PitchClass::G,
                accidental: None
            })
        );
        assert_eq!(Note::new("$"), Err("unknown pitch".to_string()));
        assert_eq!(Note::new("E%"), Err("unknown accidental".to_string()));
    }

    #[test]
    fn test_note_position() {
        assert_eq!(Note::new("Gbb").unwrap().position(), -12);
        assert_eq!(Note::new("A#").unwrap().position(), 11);
    }
}
