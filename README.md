# cue

This crate provides Rust bindings for the [libcue](https://github.com/lipnitsk/libcue) library, which supports parsing and interpreting [CUE sheets](https://en.wikipedia.org/wiki/Cue_sheet_(computing)). The CUE sheet format is commonly used for authoring CDs and to store copies of ripped CDs.

This repository contains two crates: `cue-sys`, which contains raw bindings for the original C API, and `cue`, which provides a higher-level, more Rustic interface.

## Example

Here's a simple example of how to use this crate using a sample CUE sheet:

```rust
use cue::cd::{CD, DiscMode};
use cue::track::{TrackMode, TrackSubMode};

let cue_sheet = "FILE \"example.img\" BINARY
  TRACK 01 MODE1/2352
    INDEX 01 00:00:00
  TRACK 02 AUDIO
    PREGAP 00:02:00
    INDEX 01 58:41:36
  TRACK 03 AUDIO
    INDEX 00 61:06:08
    INDEX 01 61:08:08
";

let cd = CD::parse(cue_sheet.to_string()).unwrap();

println!("Number of tracks: {}", cd.get_track_count());
let mode = match cd.get_mode() {
    DiscMode::CD_DA => "CD-DA",
    DiscMode::CD_ROM => "CD-ROM",
    DiscMode::CD_ROM_XA => "CD-ROM XA",
};
println!("Mode: {}", mode);
println!("");

for (index, track) in cd.tracks().iter().enumerate() {
    println!("Track {}", index + 1);
    println!("Filename: {}", track.get_filename());
    println!("Start: {}", track.get_start());
    println!("Length: {}", track.get_length());
    println!("Pregap: {}", track.get_zero_pre());
    println!("Postgap: {}", track.get_zero_post());
    println!("");
}
```

## Contributing

If you have trouble using this crate, feel free to file an issue asking for help. I'll do my best to help you out!

Pull requests are very welcome. If you're new to open source, or to Rust, just let me know and I'll be glad to help you through the process of contributing!

## License

Both crates are licensed under the GPL 2.0, which is the same license used by libcue.
