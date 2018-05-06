use std::ffi::CString;
use libc;
use cd_text::CDText;
use libcue_sys as libcue;
use libcue_sys::{TrackMode, TrackSubMode, TrackFlag};
use rem::REM;

pub struct Track {
    track: *mut libcue::TrackPointer,
}

impl Track {
    pub fn from(pointer: *mut libcue::TrackPointer) -> Track {
        return Track {
            track: pointer,
        };
    }

    pub fn get_filename(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::track_get_filename(self.track);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    pub fn get_start(&self) -> i64 {
        unsafe {
            return libcue::track_get_start(self.track) as i64;
        }
    }

    pub fn get_length(&self) -> i64 {
        unsafe {
            return libcue::track_get_length(self.track) as i64;
        }
    }

    pub fn get_mode(&self) -> TrackMode {
        unsafe {
            return libcue::track_get_mode(self.track);
        }
    }

    pub fn get_sub_mode(&self) -> TrackSubMode {
        unsafe {
            return libcue::track_get_sub_mode(self.track);
        }
    }

    pub fn flag_is_set(&self, flag: TrackFlag) -> bool {
        let result;
        unsafe {
            result = libcue::track_is_set_flag(self.track, flag);
        }
        return result == 1;
    }

    pub fn get_zero_pre(&self) -> i64 {
        unsafe {
            return libcue::track_get_zero_pre(self.track) as i64;
        }
    }

    pub fn get_zero_post(&self) -> i64 {
        unsafe {
            return libcue::track_get_zero_post(self.track) as i64;
        }
    }

    pub fn get_isrc(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::track_get_isrc(self.track);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    pub fn get_index(&self, index: isize) -> isize {
        unsafe {
            return libcue::track_get_index(self.track, index as libc::c_int) as isize;
        }
    }

    pub fn get_cdtext(&self) -> CDText {
        let cdtext;
        unsafe {
            cdtext = libcue::track_get_cdtext(self.track);
        }
        return CDText::from(cdtext);
    }

    pub fn get_rem(&self) -> REM {
        let rem;
        unsafe {
            rem = libcue::track_get_rem(self.track);
        }
        return REM::from(rem);
    }
}
