use std::ffi::CString;

use cue_sys as libcue;
pub use cue_sys::{TrackFlag, TrackMode, TrackSubMode};
use libc;

use crate::cd_text::CDText;
use crate::rem::REM;

/// The Track struct represents a single track within a CD.
/// A track can be one of several types, represented by the
/// `TrackMode` struct, and a CD can mix and match tracks of
/// different types in arbitrary orders.
///
/// The CUE sheet format represents its time codes
/// in a `MM:SS:FR` format, which is based on the `MIN`, `SEC` and `FRAC`
/// fields defined by the CD standard. `FRAC` is a fraction
/// representing 1/75th of a second, which is the size of a single
/// CD sector.
/// The timecodes returned by this struct have been parsed into
/// sequential sector counts.
///
/// Aside from the track itself, this struct also represents the
/// regions immediately before and after the track. These regions
/// of a disc, known as the pregap and postgap, are not contained
/// within any track but do have their own timecodes. They are
/// usually silent but occasionally contain sound as a form of
/// easter egg.
///
/// The order content looks like this:
/// * Pregap (if present)
/// * Track
/// * Postgap (if present)
/// * Next track's pregap (if present)
/// ...
///
/// Aside from the pregap and postgap, a track can consist of up
/// to 99 indices, where the start of the track is index 01.
/// Indices higher than 1 are very rarely used, and this
/// struct currently doesn't support representing them.
pub struct Track {
    track: *mut libcue::TrackPointer,
}

impl Track {
    pub fn from(pointer: *mut libcue::TrackPointer) -> Track {
        return Track { track: pointer };
    }

    /// Returns the filename of the file containing this track.
    /// The track might be stored standalone in its own file, or
    /// as part of a large file containing the contents of the entire disc.
    pub fn get_filename(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::track_get_filename(self.track);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    /// Returns the start address of this track in sectors.
    pub fn get_start(&self) -> i64 {
        unsafe {
            return libcue::track_get_start(self.track) as i64;
        }
    }

    /// Returns the length of this track in sectors.
    pub fn get_length(&self) -> i64 {
        unsafe {
            return libcue::track_get_length(self.track) as i64;
        }
    }

    /// Returns the mode of this track as a `TrackMode` variant.
    pub fn get_mode(&self) -> TrackMode {
        unsafe {
            return libcue::track_get_mode(self.track);
        }
    }

    /// Returns a `TrackSubMode` variant indicating whether the format
    /// of subchannel data provided for this track.
    ///
    /// The BIN/CUE format does not normally provide subchannel data,
    /// but when it does it provides data for subchannels R through W
    /// in either a "raw" or a "cooked" format.
    pub fn get_sub_mode(&self) -> TrackSubMode {
        unsafe {
            return libcue::track_get_sub_mode(self.track);
        }
    }

    /// Queries whether a given TrackFlag is set for the track.
    /// These indicate special properties such as whether a track
    /// contains data, is copy-protected, contains quadraphonic audio,
    /// and so on.
    pub fn flag_is_set(&self, flag: TrackFlag) -> bool {
        let result;
        unsafe {
            result = libcue::track_is_set_flag(self.track, flag);
        }
        return result == 1;
    }

    /// Returns the length of the track's pregap, in sectors.
    /// The pregap is a region contained before the track proper.
    /// It almost always consists of two seconds of silence, though
    /// it can be any length and contain any data; this is occasionally
    /// used for secret tracks.
    pub fn get_zero_pre(&self) -> i64 {
        unsafe {
            return libcue::track_get_zero_pre(self.track) as i64;
        }
    }

    /// Returns the length of the track's postgap, in sectors.
    /// Like the pregap, the postgap is a region that comes after
    /// the end of a track and before the next one; if present,
    /// it almost always consists of silence.
    pub fn get_zero_post(&self) -> i64 {
        unsafe {
            return libcue::track_get_zero_post(self.track) as i64;
        }
    }

    /// Returns the [International Standard Recording Code (ISRC)](https://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    /// for this track, if defined.
    /// While some CDs use ISRCs, they are not common and many
    /// CUE sheets leave them out even if the CD had them.
    pub fn get_isrc(&self) -> Option<String> {
        let c_string;
        unsafe {
            let raw_string = libcue::track_get_isrc(self.track);
            if raw_string.is_null() {
                return None;
            }
            c_string = CString::from_raw(raw_string);
        }
        return Some(c_string.to_string_lossy().into_owned());
    }

    /// Returns the track number.
    pub fn get_index(&self, index: isize) -> isize {
        unsafe {
            return libcue::track_get_index(self.track, index as libc::c_int) as isize;
        }
    }

    /// Returns a CDText object containing any CD-TEXT data for this track, if present.
    pub fn get_cdtext(&self) -> CDText {
        let cdtext;
        unsafe {
            cdtext = libcue::track_get_cdtext(self.track);
        }
        return CDText::from(cdtext);
    }

    /// Returns a REM object containing any comments for this track, if present.
    pub fn get_rem(&self) -> REM {
        let rem;
        unsafe {
            rem = libcue::track_get_rem(self.track);
        }
        return REM::from(rem);
    }
}
