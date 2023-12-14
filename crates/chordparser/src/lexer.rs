use logos::Logos;

// https://stackoverflow.com/questions/11229080/regex-for-matching-a-music-chord

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"\s+")]
pub enum Token {
    #[regex(r"[A-G](b|bb|#|##)?(m|M7|m7|sus|dim|aug)?", priority=2)]
    Chord,

    #[token("[")]
    LSqBracket,

    #[token("]")]
    RSqBracket,

    #[regex("[a-zA-Z]+")]
    Lyrics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_chord() {
        let mut lex = Token::lexer("[C#m7]");

        assert_eq!(lex.next(), Some(Ok(Token::LSqBracket)));
        assert_eq!(lex.span(), 0..1);
        assert_eq!(lex.slice(), "[");

        assert_eq!(lex.next(), Some(Ok(Token::Chord)));
        assert_eq!(lex.span(), 1..5);
        assert_eq!(lex.slice(), "C#m7");

        assert_eq!(lex.next(), Some(Ok(Token::RSqBracket)));
        assert_eq!(lex.span(), 5..6);
        assert_eq!(lex.slice(), "]");
    }

    #[test]
    fn lex_chords_and_lyrics() {
        let mut lex = Token::lexer("Never gonna [BbM7]give you up");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 0..5);
        assert_eq!(lex.slice(), "Never");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 6..11);
        assert_eq!(lex.slice(), "gonna");

        assert_eq!(lex.next(), Some(Ok(Token::LSqBracket)));
        assert_eq!(lex.span(), 12..13);
        assert_eq!(lex.slice(), "[");

        assert_eq!(lex.next(), Some(Ok(Token::Chord)));
        assert_eq!(lex.span(), 13..17);
        assert_eq!(lex.slice(), "BbM7");

        assert_eq!(lex.next(), Some(Ok(Token::RSqBracket)));
        assert_eq!(lex.span(), 17..18);
        assert_eq!(lex.slice(), "]");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 18..22);
        assert_eq!(lex.slice(), "give");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 23..26);
        assert_eq!(lex.slice(), "you");

        assert_eq!(lex.next(), Some(Ok(Token::Lyrics)));
        assert_eq!(lex.span(), 27..29);
        assert_eq!(lex.slice(), "up");

        assert_eq!(lex.next(), None);
    }
}