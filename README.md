# caramell

Caramell is a lyric and chord sheet formatter.
At the moment, it can take this:

```
Never gonna [Ebm9]give you [Ab]up
Never gonna [Fm7]let you [Bbm]down
Never gonna [Ebm9]run a[Ab]round and de-[Fm7]sert [Bbm]you
```

And turn it into this:

```text
            Ebm9     Ab
Never gonna give you up
            Fm7     Bbm
Never gonna let you down
            Ebm9 Ab           Fm7  Bbm
Never gonna run around and de-sert you
```

## Motivation

This was just a fun coding project to learn about 1) lexing, parsing, etc. and 2) algorithms for musical transposition. In my opinion, this tool isn't very practically useful so much as it is pedagogically useful.

- It's a lot faster to just write out the lyrics and chords in Word or other text editor,
even if lining up the words and chords can get kind of annoying.
- As for transposition, I find it's a much more useful skill to be able to transpose on the fly with your instrument, rather than rely on a computer to do it for your lyric and chord sheets.

## Inspiration

* [(Github) ChordPro/chordpro](https://github.com/ChordPro/chordpro)
* [(GitHub) joseluiscd/chordpro-rs](https://github.com/joseluiscd/chordpro-rs)

## Music Theory

### Notation

See [https://playingchordcharts.com/chords#notation](https://playingchordcharts.com/chords#notation) for more details on chord notation.

Some ideas were also taken from the [ChordPro](https://www.chordpro.org/chordpro/chordpro-introduction/) specification.

[Gallery of Interesting Music Notation](https://homes.luddy.indiana.edu/donbyrd/InterestingMusicNotation.html)

### Transposition

[Music Transposition in Musescore](https://davidbolton.info/articles/interval-transposition-in-musescore/)

[Algorithm for Transposing Chords](https://music.stackexchange.com/questions/40041/algorithm-for-transposing-chords-between-keys)

[MuseScore Tonal Pitch Class](https://musescore.org/en/handbook/developers-handbook/plugin-development/tonal-pitch-class-enum)

[Line of Fifths](https://music.stackexchange.com/questions/17318/general-procedure-for-determining-the-name-of-an-interval-given-a-major-key-di/17365#17365)