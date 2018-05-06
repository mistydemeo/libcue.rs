use std::ffi::CString;
use libc;
use libcue_sys as libcue;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum RemType {
    Date,
    ReplayGainAlbumGain,
    ReplayGainAlbumPeak,
    ReplaygainTrackGain,
    ReplaygainTrackPeak,
    End,
}

pub struct REM {
    rem: *mut libcue::RemPointer,
}

impl REM {
    pub fn from(pointer: *mut libcue::RemPointer) -> REM {
        return REM {
            rem: pointer,
        };
    }

    pub fn read(&self, index: usize) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::rem_get(index as libc::c_uint, self.rem);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }
}
