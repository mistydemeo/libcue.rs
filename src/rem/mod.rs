use std::ffi::CString;

use libc;
use cue_sys as libcue;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum RemType {
    Date,
    ReplayGainAlbumGain,
    ReplayGainAlbumPeak,
    ReplayGainTrackGain,
    ReplayGainTrackPeak,
    End,
}

/// Represents a comment on a `CD` or Track.
/// This field is usually just used to store arbitrary human-readable comments,
/// but is also used by some programs to include custom metadata.
///
/// [`CD`]: ../struct.CD.html
pub struct REM {
    rem: *mut libcue::RemPointer,
}

impl REM {
    pub fn from(pointer: *mut libcue::RemPointer) -> REM {
        return REM {
            rem: pointer,
        };
    }

    /// Returns the comment represented by this struct as a string, if present.
    pub fn read(&self, index: usize) -> Option<String> {
        let c_string;
        unsafe {
            let raw_string = libcue::rem_get(index as libc::c_uint, self.rem);
            if raw_string.is_null() {
                return None;
            }
            c_string = CString::from_raw(raw_string);
        }
        return Some(c_string.to_string_lossy().into_owned());
    }
}
