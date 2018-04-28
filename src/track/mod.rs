use std::ffi::CString;
use libc;
use cd_text::CDText;
use raw;
use rem::REM;

#[repr(C)]
pub enum TrackMode {
    Audio,
    Mode1,
    Mode1RAW,
    Mode2,
    Mode2Form1,
    Mode2Form2,
    Mode2FormMix,
    Mode2Raw,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum TrackSubMode {
    RW,
    RW_RAW,
}

#[repr(C)]
pub enum TrackFlag {
    None,
    PreEmphasis,
    CopyPermitted,
    Data,
    FourChannel,
    SCMS,
    Any,
}

pub struct Track {
    track: *mut raw::TrackPointer,
}

impl Track {
    pub fn from(pointer: *mut raw::TrackPointer) -> Track {
        return Track {
            track: pointer,
        };
    }

    pub fn get_filename(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = raw::track_get_filename(self.track);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    pub fn get_start(&self) -> u64 {
        unsafe {
            return raw::track_get_start(self.track) as u64;
        }
    }

    pub fn get_length(&self) -> u64 {
        unsafe {
            return raw::track_get_length(self.track) as u64;
        }
    }

    pub fn get_mode(&self) -> TrackMode {
        unsafe {
            return raw::track_get_mode(self.track);
        }
    }

    pub fn get_sub_mode(&self) -> TrackSubMode {
        unsafe {
            return raw::track_get_sub_mode(self.track);
        }
    }

    pub fn flag_is_set(&self, flag: TrackFlag) -> bool {
        let result;
        unsafe {
            result = raw::track_is_set_flag(self.track, flag);
        }
        return result == 1;
    }

    pub fn get_zero_pre(&self) -> u64 {
        unsafe {
            return raw::track_get_zero_pre(self.track) as u64;
        }
    }

    pub fn get_zero_post(&self) -> u64 {
        unsafe {
            return raw::track_get_zero_post(self.track) as u64;
        }
    }

    pub fn get_isrc(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = raw::track_get_isrc(self.track);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    pub fn get_index(&self, index: usize) -> usize {
        unsafe {
            return raw::track_get_index(self.track, index as libc::c_int) as usize;
        }
    }

    pub fn get_cdtext(&self) -> CDText {
        let cdtext;
        unsafe {
            cdtext = raw::track_get_cdtext(self.track);
        }
        return CDText::from(cdtext);
    }

    pub fn get_rem(&self) -> REM {
        let rem;
        unsafe {
            rem = raw::track_get_rem(self.track);
        }
        return REM::from(rem);
    }
}
