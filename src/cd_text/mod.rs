use std::ffi::CString;

use libcue_sys as libcue;
pub use libcue_sys::PTI;

/// Represents [CD-TEXT](https://en.wikipedia.org/wiki/CD-Text)
/// data stored in either a [`CD`] or a single [`Track`].
///
/// CD-TEXT is a simple metadata format embedded into an audio CD's
/// subcode data which is streamed alongside the CD content itself.
/// It provides a basic way for discs to include information such as
/// track titles, genres, and basic credits.
/// CUE sheets can contain parsed plaintext representations of CD-TEXT
/// data.
///
/// [`CD`]: ../cd/struct.CD.html
/// [`Track`]: ../track/struct.Track.html
pub struct CDText {
    cdtext: *mut libcue::CdtextPointer,
}

impl CDText {
    pub fn from(pointer: *mut libcue::CdtextPointer) -> CDText {
        return CDText {
            cdtext: pointer,
        };
    }

    /// Returns the CD-TEXT data represented by this struct as a string, if present.
    pub fn read(&self, pack_type: PTI) -> Option<String> {
        let c_string;
        unsafe {
            let raw_string = libcue::cdtext_get(pack_type, self.cdtext);
            if raw_string.is_null() {
                return None;
            }
            c_string = CString::from_raw(raw_string);
        }
        return Some(c_string.to_string_lossy().into_owned());
    }
}
