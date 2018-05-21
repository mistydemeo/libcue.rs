//! This crate contains high-level mappings for the [libcue](https://github.com/lipnitsk/libcue)
//! CUE sheet parsing library.
//! The [CUE sheet](https://en.wikipedia.org/wiki/Cue_sheet_(computing)) format is a
//! high-level representation of the table of contents of a CD audio or CD-ROM disc,
//! and allows for a simple human-readable way of either authoring a disc or storing
//! a disc image based off of an actual CD.
//!
//! While this crate includes some documentation on the CUE format and CDs, this
//! documentation assumes that the reader is already familiar with the CD and CD-ROM
//! formats. The Wikipedia articles for [Compact Disc](https://en.wikipedia.org/wiki/Compact_disc),
//! [CD-ROM](https://en.wikipedia.org/wiki/CD-ROM) and [CUE sheet](https://en.wikipedia.org/wiki/Cue_sheet_(computing))
//! are good accessible introductions.

extern crate libc;
extern crate libcue_sys;

/// The `CD` struct represents a disc, and is the entry point to parsing a CUE sheet.
pub mod cd;
/// The `CDText` struct represents [CD-TEXT](https://en.wikipedia.org/wiki/CD-Text) data
/// stored alongside tracks in a CUE sheet.
pub mod cd_text;
/// The `REM` struct represents comments within a CUE sheet.
pub mod rem;
/// The `Track` struct represents a single track of a CD.
pub mod track;
