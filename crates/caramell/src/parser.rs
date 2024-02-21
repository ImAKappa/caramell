use crate::lexer::Token;
use logos::Logos;

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
    pub line: usize,
}

impl Phrase {
    pub fn new(
        lyrics: String,
        start: usize,
        end: usize,
        chord: Option<Chord>,
        line: usize,
    ) -> Self {
        Self {
            lyrics,
            start,
            end,
            chord,
            line,
        }
    }

    pub fn empty() -> Self {
        Self {
            lyrics: String::new(),
            start: 0,
            end: 0,
            chord: None,
            line: 0,
        }
    }
}

pub fn parse(song: String) -> Result<Vec<Phrase>, String> {
    let mut lex = Token::lexer(&song);
    let mut phrases: Vec<Phrase> = Vec::new();
    let mut phrase = Phrase::empty();
    let mut line: usize = 0;
    loop {
        if let Some(token) = lex.next() {
            match token {
                Ok(Token::Lyrics) => {
                    phrase.lyrics = lex.slice().to_string();
                    // println!("{:?}", phrase.lyrics);
                    phrase.start = lex.span().start;
                    phrase.end = lex.span().end;
                    phrase.line = line;
                }
                Ok(Token::LSqBracket) => {
                    // Terminate current phrase
                    phrases.push(phrase.clone());
                    // Reset phrase
                    phrase = Phrase::empty();
                }
                Ok(Token::Chord) => {
                    // Start new phrase
                    phrase.chord = Some(Chord::new(lex.slice().to_string()));
                }
                Ok(Token::RSqBracket) => {}
                Ok(Token::NewLine) => {
                    phrases.push(phrase.clone());
                    phrase = Phrase::empty();
                    line += 1;
                }
                Err(()) => return Err(format!("Token error: {:?}", lex.slice())),
            };
        } else {
            // Store last phrase
            phrases.push(phrase.clone());
            break;
        }
    }
    Ok(phrases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chords() {
        assert_eq!(
            parse("[C][G][Am][F]".to_string()),
            Ok(vec![
                Phrase {
                    lyrics: "".to_string(),
                    start: 0,
                    end: 0,
                    chord: None,
                    line: 0
                },
                Phrase {
                    lyrics: "".to_string(),
                    start: 0,
                    end: 0,
                    chord: Some(Chord::new("C".to_string())),
                    line: 0
                },
                Phrase {
                    lyrics: "".to_string(),
                    start: 0,
                    end: 0,
                    chord: Some(Chord::new("G".to_string())),
                    line: 0
                },
                Phrase {
                    lyrics: "".to_string(),
                    start: 0,
                    end: 0,
                    chord: Some(Chord::new("Am".to_string())),
                    line: 0
                },
                Phrase {
                    lyrics: "".to_string(),
                    start: 0,
                    end: 0,
                    chord: Some(Chord::new("F".to_string())),
                    line: 0
                },
            ])
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
            Ok(vec![
                Phrase {
                    lyrics: "Never gonna ".to_string(),
                    start: 0,
                    end: 12,
                    chord: None,
                    line: 0
                },
                Phrase {
                    lyrics: "give you ".to_string(),
                    start: 18,
                    end: 27,
                    chord: Some(Chord::new("Ebm9".to_string())),
                    line: 0
                },
                Phrase {
                    lyrics: "up".to_string(),
                    start: 31,
                    end: 33,
                    chord: Some(Chord::new("Ab".to_string())),
                    line: 0
                },
                Phrase {
                    lyrics: "Never gonna ".to_string(),
                    start: 34,
                    end: 46,
                    chord: None,
                    line: 1
                },
                Phrase {
                    lyrics: "let you ".to_string(),
                    start: 51,
                    end: 59,
                    chord: Some(Chord::new("Fm7".to_string())),
                    line: 1
                },
                Phrase {
                    lyrics: "down".to_string(),
                    start: 64,
                    end: 68,
                    chord: Some(Chord::new("Bbm".to_string())),
                    line: 1
                },
            ])
        );
    }
}
