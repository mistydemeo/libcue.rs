use std::ffi::CString;
use libcue_sys as libcue;
use libcue_sys::PTI;

pub struct CDText {
    cdtext: *mut libcue::CdtextPointer,
}

impl CDText {
    pub fn from(pointer: *mut libcue::CdtextPointer) -> CDText {
        return CDText {
            cdtext: pointer,
        };
    }

    pub fn read(&self, pack_type: PTI) -> String {
        let c_string;
        unsafe {
            let raw_string = libcue::cdtext_get(pack_type, self.cdtext);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }
}
