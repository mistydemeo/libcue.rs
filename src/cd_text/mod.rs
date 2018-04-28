use std::ffi::CString;
use raw;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum PTI {
    Title,
    Performer,
    Songwriter,
    Composer,
    Arranger,
    Message,
    DiscID,
    Genre,
    TOCInfo1,
    TOCInfo2,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    UPC_ISRC,
    SizeInfo,
    End,
}

pub struct CDText {
    cdtext: *mut raw::CdtextPointer,
}

impl CDText {
    pub fn from(pointer: *mut raw::CdtextPointer) -> CDText {
        return CDText {
            cdtext: pointer,
        };
    }

    pub fn read(&self, pack_type: PTI) -> String {
        let c_string;
        unsafe {
            let raw_string = raw::cdtext_get(pack_type, self.cdtext);
            c_string = CString::from_raw(raw_string);
        }
        return c_string.to_string_lossy().into_owned();
    }
}
