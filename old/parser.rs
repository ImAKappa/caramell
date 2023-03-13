use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::error::Error;

fn directive(i: &str) -> IResult<&str, &str> {
    let (i, _) = tag("{")(i)?;
    let (i, slice_directive) = take_while1(|c: char| c.is_alphabetic() || c == '_')(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, slice_directive))
}

fn is_valid_note(c: char) -> bool {
    match c {
        'A'..='G' => true,
        _ => false,
    }
}

fn is_valid_accidental(a: char) -> bool {
    match a {
        '#' | 'b' => true,
        _ => false,
    }
}

fn is_valid_chord_symbol(c: char) -> bool {
    is_valid_accidental(c) || is_valid_note(c) || c == '/'
}

fn chord(i: &str) -> IResult<&str, &str> {
    let (i, _) = tag("[")(i)?;
    let (i, slice_chord) = take_while1(|c: char| is_valid_chord_symbol(c))(i)?;
    let (i, _) = tag("]")(i)?;
    Ok((i, slice_chord))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directive() {
        // Directive can only have alphabetic
        assert!(directive("{Hello World!}").is_err());
        assert!(directive("{end_of_chorus123}").is_err());
        assert_eq!(directive("{start_of_chorus}"), Ok(("", "start_of_chorus")));
    }

    #[test]
    fn test_chord() {
        assert!(chord("C").is_err(), "Must be surrounded by brackets");
        // Sorry, German notation...
        assert!(chord("[H]").is_err(), "Only letters A..G are accepted");
        assert!(chord("[]").is_err(), "Must contain chord");
        assert!(chord("[#]").is_err(), "Must contain note first, then (maybe) accidental");
        assert_eq!(chord("[E]"), Ok(("", "E")));
        assert_eq!(chord("[E/G#]"), Ok(("", "E/G#")));
    }
}