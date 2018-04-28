use std::ffi::CString;
use libc;
use raw;

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
    rem: *mut raw::RemPointer,
}

impl REM {
    pub fn from(pointer: *mut raw::RemPointer) -> REM {
        return REM {
            rem: pointer,
        };
    }

    pub fn read(&self, index: usize) -> String {
        let c_string;
        unsafe {
            let raw_string = raw::rem_get(index as libc::c_uint, self.rem);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }
}
