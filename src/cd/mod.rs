use std::ffi::{CString, NulError};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;

use libc;
use libcue_sys as libcue;
pub use libcue_sys::DiscMode;

use cd_text::CDText;
use rem::REM;
use track::Track;

/// The CD struct represents the entirety of a CD, which is the core unit of
/// a CUE sheet. This struct contains the parsing methods used as the primary
/// entry point to libcue's functionality.
///
/// A CD can be a pure CD audio disc, a pure data disc, or a mixed-mode disc
/// containing both data and audio tracks in arbitrary order. A CD will
/// always have at least one track.
///
/// Here's an example of a simple function which parses a CUE sheet and
/// prints information about its contents:
///
/// ```rust, ignore
/// let cd = CD::parse_file(path).unwrap();
///
/// println!("Number of tracks: {}", cd.get_track_count());
/// let mode = match cd.get_mode() {
///     DiscMode::CD_DA => "CD-DA",
///     DiscMode::CD_ROM => "CD-ROM",
///     DiscMode::CD_ROM_XA => "CD-ROM XA",
/// };
/// println!("Mode: {}", mode);
/// println!("");
///
/// for (index, track) in cd.tracks().iter().enumerate() {
///     println!("Track {}", index + 1);
///     println!("Filename: {}", track.get_filename());
///     println!("Mode: {}", track_mode);
///     println!("Start: {}", track.get_start());
///     println!("Length: {}", track.get_length());
///     println!("Pregap: {}", track.get_zero_pre());
///     println!("Postgap: {}", track.get_zero_post());
///     println!("");
/// }
/// ```
pub struct CD {
    cd: *mut libcue::CdPointer,
}

impl CD {
    /// Parses a string containing a CUE sheet and returns a `CD` struct.
    ///
    /// # Errors
    /// Returns a `NulError` if the provided string contains any null bytes.
    pub fn parse(string: String) -> Result<CD, NulError> {
        let c_string = CString::new(string)?;
        let cd;
        unsafe {
            cd = libcue::cue_parse_string(c_string.as_ptr());
        }
        let cd_type = CD {
            cd: cd,
        };
        return Ok(cd_type);
    }

    /// Reads the file contained at `path` and parses it like the [`parse`](#method.parse) function
    /// above.
    pub fn parse_file(path: PathBuf) -> Result<CD, io::Error> {
        let mut cue_sheet = vec![];
        File::open(&path)?.read_to_end(&mut cue_sheet)?;
        return Ok(CD::parse(String::from_utf8_lossy(&cue_sheet).into_owned()).unwrap());
    }

    /// Returns a `DiscMode` value indicating the type of disc represented by this
    /// CUE sheet.
    /// Individual tracks also have types; see [`Track.get_mode`] and [`Track.get_sub_mode`].
    ///
    /// [`Track.get_mode`]: ../track/struct.Track.html#method.get_mode
    /// [`Track.get_sub_mode`]: ../track/struct.Track.html#method.get_sub_mode
    pub fn get_mode(&self) -> DiscMode {
        unsafe {
            return libcue::cd_get_mode(self.cd);
        }
    }

    /// Returns the path on disc to the sidecar metadata file containing CD-TEXT
    /// metadata, if any.
    pub fn get_cdtextfile(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::cd_get_cdtextfile(self.cd);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    /// Returns the total number of tracks in this CD.
    pub fn get_track_count(&self) -> isize {
        unsafe {
            return libcue::cd_get_ntrack(self.cd) as isize;
        }
    }

    /// Returns a [`Track`] struct for the track at the requested index.
    /// Note that track numbering starts from 1; there is no track 0.
    ///
    /// # Errors
    /// If the requested track doesn't exist in the CD, returns `Err`
    /// with a string containing an error message.
    ///
    /// [`Track`]: ../track/struct.Track.html
    pub fn get_track(&self, index: isize) -> Result<Track, String> {
        let track_count = self.get_track_count();
        if index > track_count {
            return Err(format!("Invalid index; CD has {} tracks", track_count));
        }

        let track;
        unsafe {
            track = libcue::cd_get_track(self.cd, index as libc::c_int);
        }

        return Ok(Track::from(track));
    }

    /// Returns a `Vec` containing every track in the CD.
    pub fn tracks(&self) -> Vec<Track> {
        let mut tracks = vec![];
        let mut index = 1;
        while index <= self.get_track_count() {
            tracks.push(self.get_track(index).unwrap());
            index += 1;
        }

        return tracks;
    }

    /// Returns a [`CDText`] representing the CD-TEXT data stored within
    /// this CUE sheet.
    ///
    /// [`CDText`]: ../cd_text/struct.CDText.html
    pub fn get_cdtext(&self) -> CDText {
        let cdtext;
        unsafe {
            cdtext = libcue::cd_get_cdtext(self.cd);
        }
        return CDText::from(cdtext);
    }

    /// Returns a [`REM`] representing the comments stored within
    /// this CUE sheet.
    ///
    /// [`REM`]: ../rem/struct.REM.html
    pub fn get_rem(&self) -> REM {
        let rem;
        unsafe {
            rem = libcue::cd_get_rem(self.cd);
        }
        return REM::from(rem);
    }
}

impl Drop for CD {
    fn drop(&mut self) {
        unsafe {
            libcue::cd_delete(self.cd);
        }
    }
}
