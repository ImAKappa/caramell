use crate::parser::Lines;

fn calc_chord_padding(lyric: &str, chord: &str) -> usize {
    let mut padding = lyric.len();
    padding = padding.checked_sub(chord.len()).unwrap_or(0);
    padding
}

pub fn fmt_lyrics_and_chords(lines: Lines) -> String {
    let mut fmt_song = String::new();
    for (_, phrases) in lines.lines.iter() {
        // TODO: Handle case with newline/empty line - no need to construct a chord line
        let mut chord_line = String::new();
        let mut lyric_line = String::new();
        for p in phrases.into_iter() {
            let chord = p
                .chord
                .as_ref()
                .map_or("".to_string(), |c| c.chord.to_owned());
            chord_line.push_str(&chord);
            let padding = &" ".repeat(calc_chord_padding(&p.lyrics, &chord));
            chord_line.push_str(padding);
            lyric_line.push_str(&p.lyrics);
        }
        fmt_song.push_str(&chord_line);
        fmt_song.push_str("\n");
        fmt_song.push_str(&lyric_line);
        fmt_song.push_str("\n");
    }
    fmt_song
}

#[cfg(test)]
mod tests {
    use crate::parser::Chord;
    use crate::parser::Phrase;

    use super::*;

    #[test]
    fn test_calc_chord_padding_with_lyrics_and_chord() {
        assert_eq!(2, calc_chord_padding("Hi there", "C#maj7"));
    }

    #[test]
    fn test_calc_chord_padding_with_lyrics_no_chord() {
        assert_eq!(8, calc_chord_padding("Hi there", ""));
    }

    #[test]
    fn test_calc_chord_padding_with_empty_lyric_no_chord() {
        assert_eq!(0, calc_chord_padding("", ""));
    }

    #[test]
    fn test_calc_chord_padding_with_empty_lyric_and_chord() {
        assert_eq!(0, calc_chord_padding("", "C#maj7"));
    }

    #[test]
    fn test_fmt_lyrics_and_chords() {
        let mut lines = Lines::new();
        lines.add_phrase(
            0,
            Phrase::new(
                "Hi there".to_string(),
                0,
                8,
                Some(Chord::new("C#maj7".to_string())),
            ),
        );
        lines.add_phrase(
            1,
            Phrase::new(
                "Bye there".to_string(),
                0,
                8,
                Some(Chord::new("Dsus2".to_string())),
            ),
        );

        assert_eq!(
            fmt_lyrics_and_chords(lines),
            "C#maj7  \nHi there\nDsus2    \nBye there\n".to_string()
        )
    }
}
