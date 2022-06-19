# midify-chords
`midify-chords` is a command line tool that converts plaintext chords to [MIDI](https://en.wikipedia.org/wiki/MIDI).

**Example:**
```
# From STDIN
echo "C Am Dm G | C Am Dm G" | midify-chords -o song.mid
# From file
midify-chords -o song.mid < song.txt
# From file, 200bpm
midify-chords -o song.mid -b 200 < song.txt
```

Using C as an example, the supported chords are
```
C Cm Cdim Caug Csus
```
## Why?
The idea is to have a simple way to write down a chord progression and convert it to a flexible format. MIDI allows you to listen to the chords, or import them into notation software such as [MuseScore](https://musescore.org/) and expand on your musical idea.
## Resources
There is a great summary of the MIDI specification at [somascape.org](http://www.somascape.org/midi/tech/mfile.html).

Another great resource for learning about MIDI is writing some music in [MuseScore](https://musescore.org/), exporting to MIDI and inspecting the [hex dump](https://en.wikipedia.org/wiki/Hex_dump).

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
