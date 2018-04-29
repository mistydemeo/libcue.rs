use std::ffi::{CString, NulError};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;

use libc;

use cd_text::CDText;
use raw;
use rem::REM;
use track::Track;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum DiscMode {
    CD_DA,
    CD_ROM,
    CD_ROM_XA,
}

pub struct CD {
    cd: *mut raw::CdPointer,
}

impl CD {
    pub fn parse(string: String) -> Result<CD, NulError> {
        let c_string = CString::new(string)?;
        let cd;
        unsafe {
            cd = raw::cue_parse_string(c_string.as_ptr());
        }
        let cd_type = CD {
            cd: cd,
        };
        return Ok(cd_type);
    }

    pub fn parse_file(path: PathBuf) -> Result<CD, io::Error> {
        let mut cue_sheet = vec![];
        File::open(&path)?.read_to_end(&mut cue_sheet)?;
        return Ok(CD::parse(String::from_utf8_lossy(&cue_sheet).into_owned()).unwrap());
    }

    pub fn get_mode(&self) -> DiscMode {
        unsafe {
            return raw::cd_get_mode(self.cd);
        }
    }

    pub fn get_cdtextfile(&self) -> String {
        let c_string;
        unsafe {
            let raw_string = raw::cd_get_cdtextfile(self.cd);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }

    pub fn get_track_count(&self) -> isize {
        unsafe {
            return raw::cd_get_ntrack(self.cd) as isize;
        }
    }

    pub fn get_track(&self, index: isize) -> Result<Track, String> {
        let track_count = self.get_track_count();
        if index > track_count {
            return Err(format!("Invalid index; CD has {} tracks", track_count));
        }

        let track;
        unsafe {
            track = raw::cd_get_track(self.cd, index as libc::c_int);
        }

        return Ok(Track::from(track));
    }

    pub fn tracks(&self) -> Vec<Track> {
        let mut tracks = vec![];
        let mut index = 1;
        while index <= self.get_track_count() {
            tracks.push(self.get_track(index).unwrap());
            index += 1;
        }

        return tracks;
    }

    pub fn get_cdtext(&self) -> CDText {
        let cdtext;
        unsafe {
            cdtext = raw::cd_get_cdtext(self.cd);
        }
        return CDText::from(cdtext);
    }

    pub fn get_rem(&self) -> REM {
        let rem;
        unsafe {
            rem = raw::cd_get_rem(self.cd);
        }
        return REM::from(rem);
    }
}

impl Drop for CD {
    fn drop(&mut self) {
        unsafe {
            raw::cd_delete(self.cd);
        }
    }
}
