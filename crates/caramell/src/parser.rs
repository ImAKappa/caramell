use crate::lexer::Token;
use logos::Logos;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub chord: String,
}

impl Chord {
    pub fn new(s: String) -> Self {
        Self { chord: s }
    }
}

/// A phrase is a subsection of a line of song
/// One phrase has only 1 or 0 chords
#[derive(Debug, Clone, PartialEq)]
pub struct Phrase {
    pub lyrics: String,
    pub start: usize,
    pub end: usize,
    pub chord: Option<Chord>,
}

impl Phrase {
    pub fn new(lyrics: String, start: usize, end: usize, chord: Option<Chord>) -> Self {
        Self {
            lyrics,
            start,
            end,
            chord,
        }
    }

    pub fn empty() -> Self {
        Self {
            lyrics: String::new(),
            start: 0,
            end: 0,
            chord: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lines {
    pub lines: HashMap<usize, Vec<Phrase>>,
}

impl Lines {
    pub fn new() -> Self {
        Self {
            lines: HashMap::new(),
        }
    }

    pub fn add_phrase(&mut self, line: usize, phrase: Phrase) {
        if let Some(phrases) = self.lines.get_mut(&line) {
            phrases.push(phrase);
        } else {
            self.lines.insert(line, vec![phrase]);
        }
    }
}

pub fn parse(song: String) -> Result<Lines, String> {
    let mut lex = Token::lexer(&song);
    let mut current_phrase = Phrase::empty();
    let mut current_line: usize = 0;
    let mut lines = Lines::new();
    loop {
        if let Some(token) = lex.next() {
            match token {
                Ok(Token::Lyrics) => {
                    current_phrase.lyrics = lex.slice().to_string();
                    current_phrase.start = lex.span().start;
                    current_phrase.end = lex.span().end;
                }
                Ok(Token::LSqBracket) => {
                    lines.add_phrase(current_line, current_phrase.clone());
                    current_phrase = Phrase::empty();
                }
                Ok(Token::Chord) => {
                    // Start new phrase
                    current_phrase.chord = Some(Chord::new(lex.slice().to_string()));
                }
                Ok(Token::RSqBracket) => {}
                Ok(Token::NewLine) => {
                    lines.add_phrase(current_line, current_phrase.clone());
                    current_phrase = Phrase::empty();
                    current_line += 1;
                }
                Err(()) => return Err(format!("Token error: {:?}", lex.slice())),
            };
        } else {
            // Store last phrase
            lines.add_phrase(current_line, current_phrase.clone());
            break;
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_phrase() {
        let mut lines = Lines::new();
        lines.add_phrase(0, Phrase::new("Hi".to_string(), 0, 0, None));
        assert_eq!(
            lines.lines,
            HashMap::from([(0 as usize, vec![Phrase::new("Hi".to_string(), 0, 0, None)])])
        )
    }

    #[test]
    fn test_parse_chords() {
        assert_eq!(
            parse("[C][G][Am][F]".to_string()),
            Ok(Lines {
                lines: HashMap::from([(
                    0,
                    vec![
                        Phrase::new("".to_string(), 0, 0, None,),
                        Phrase::new("".to_string(), 0, 0, Some(Chord::new("C".to_string()))),
                        Phrase::new("".to_string(), 0, 0, Some(Chord::new("G".to_string()))),
                        Phrase::new("".to_string(), 0, 0, Some(Chord::new("Am".to_string()))),
                        Phrase::new("".to_string(), 0, 0, Some(Chord::new("F".to_string()))),
                    ]
                )])
            })
        );
    }

    #[test]
    fn test_parse_rickroll() {
        assert_eq!(
            parse(
                r#"Never gonna [Ebm9]give you [Ab]up
Never gonna [Fm7]let you [Bbm]down"#
                    .to_string()
            ),
            Ok(Lines {
                lines: HashMap::from([
                    (
                        0,
                        vec![
                            Phrase::new("Never gonna ".to_string(), 0, 12, None),
                            Phrase::new(
                                "give you ".to_string(),
                                18,
                                27,
                                Some(Chord::new("Ebm9".to_string()))
                            ),
                            Phrase::new(
                                "up".to_string(),
                                31,
                                33,
                                Some(Chord::new("Ab".to_string()))
                            )
                        ]
                    ),
                    (
                        1,
                        vec![
                            Phrase::new("Never gonna ".to_string(), 34, 46, None),
                            Phrase::new(
                                "let you ".to_string(),
                                51,
                                59,
                                Some(Chord::new("Fm7".to_string()))
                            ),
                            Phrase::new(
                                "down".to_string(),
                                64,
                                68,
                                Some(Chord::new("Bbm".to_string()))
                            ),
                        ]
                    )
                ])
            })
        );
    }
}
