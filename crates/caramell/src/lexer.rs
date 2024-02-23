use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // https://stackoverflow.com/questions/11229080/regex-for-matching-a-music-chord
    #[regex(
        r"[A-G](b|#)?(mMaj|Maj|min|m|sus|dim|aug)?(1[0-2]|[1-9])?(/[A-G](b|#)?)?",
        priority = 2
    )]
    Chord,

    #[token("[")]
    LSqBracket,

    #[token("]")]
    RSqBracket,

    #[regex("\n|\r\n")]
    NewLine,

    // TODO: Look into multi-lingual lyric lexer: https://www.regular-expressions.info/unicode.html#prop
    #[regex("[a-zA-Z -'\",]*")]
    Lyrics,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: String, t: Token) {
        let mut lex = Token::lexer(&s);
        assert_eq!(lex.next(), Some(Ok(t)));
        assert_eq!(lex.slice(), &s);
    }

    #[test]
    fn lex_newline() {
        check("\n".to_string(), Token::NewLine);
        check("\r\n".to_string(), Token::NewLine);
    }

    #[test]
    fn lex_major_chords() {
        check("C".to_string(), Token::Chord);
        check("C#".to_string(), Token::Chord);
        check("Cb".to_string(), Token::Chord);
    }

    #[test]
    fn lex_minor_chords() {
        check("Cm".to_string(), Token::Chord);
        check("C#m".to_string(), Token::Chord);
        check("Cbm".to_string(), Token::Chord);
    }

    #[test]
    fn lex_power_chords() {
        check("C5".to_string(), Token::Chord);
        check("C#5".to_string(), Token::Chord);
        check("Cb5".to_string(), Token::Chord);
    }

    #[test]
    fn lex_6th_chords() {
        check("C6".to_string(), Token::Chord);
        check("C#6".to_string(), Token::Chord);
        check("Cb6".to_string(), Token::Chord);
    }

    #[test]
    fn lex_major_7th_chords() {
        check("CMaj7".to_string(), Token::Chord);
        check("C#Maj7".to_string(), Token::Chord);
        check("CbMaj7".to_string(), Token::Chord);
    }

    #[test]
    fn lex_minor_7th_chords() {
        check("Cm7".to_string(), Token::Chord);
        check("C#m7".to_string(), Token::Chord);
        check("Cbm7".to_string(), Token::Chord);
    }

    #[test]
    fn lex_dominant_7th_chords() {
        check("C7".to_string(), Token::Chord);
        check("C#7".to_string(), Token::Chord);
        check("Cb7".to_string(), Token::Chord);
    }

    #[test]
    fn lex_minor_major_7th_chords() {
        check("CmMaj7".to_string(), Token::Chord);
        check("C#mMaj7".to_string(), Token::Chord);
        check("CbmMaj7".to_string(), Token::Chord);
    }

    #[test]
    fn lex_suspended_chords() {
        check("Csus2".to_string(), Token::Chord);
        check("C#sus2".to_string(), Token::Chord);
        check("Cbsus2".to_string(), Token::Chord);

        check("Csus4".to_string(), Token::Chord);
        check("C#sus4".to_string(), Token::Chord);
        check("Cbsus4".to_string(), Token::Chord);
    }

    #[test]
    fn lex_augmented_chords() {
        check("Caug".to_string(), Token::Chord);
        check("C#aug".to_string(), Token::Chord);
        check("Cbaug".to_string(), Token::Chord);
    }

    #[test]
    fn lex_diminished_chords() {
        check("Cdim".to_string(), Token::Chord);
        check("C#dim".to_string(), Token::Chord);
        check("Cbdim".to_string(), Token::Chord);
    }

    #[test]
    fn lex_diminished_7th_chords() {
        check("Cdim7".to_string(), Token::Chord);
        check("C#dim7".to_string(), Token::Chord);
        check("Cbdim7".to_string(), Token::Chord);
    }

    #[test]
    fn lex_slash_chord() {
        check("C/G".to_string(), Token::Chord);
        check("C#/G#".to_string(), Token::Chord);
        check("Cb/Gb".to_string(), Token::Chord);
    }

    #[test]
    fn lex_chords_and_lyrics() {
        let mut lex = Token::lexer("Never gonna [BbMaj7]give you up");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 0..12);
        assert_eq!(lex.slice(), "Never gonna ");

        assert_eq!(lex.next(), Some(Ok(Token::LSqBracket)));
        assert_eq!(lex.span(), 12..13);
        assert_eq!(lex.slice(), "[");

        assert_eq!(lex.next(), Some(Ok(Token::Chord)));
        assert_eq!(lex.span(), 13..19);
        assert_eq!(lex.slice(), "BbMaj7");

        assert_eq!(lex.next(), Some(Ok(Token::RSqBracket)));
        assert_eq!(lex.span(), 19..20);
        assert_eq!(lex.slice(), "]");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 20..31);
        assert_eq!(lex.slice(), "give you up");

        assert_eq!(lex.next(), None);
    }
}
