use libc;
use cd::DiscMode;
use cd_text::PTI;
use track::{TrackMode, TrackSubMode, TrackFlag};

pub enum CdPointer {}
pub enum CdtextPointer {}
pub enum TrackPointer {}
pub enum RemPointer {}

#[link(name = "cue")]
extern "C" {
    pub fn cue_parse_string(contents: *const libc::c_char) -> *mut CdPointer;
    pub fn cd_delete(cd: *mut CdPointer);

    pub fn cd_get_mode(cd: *const CdPointer) -> DiscMode;
    pub fn cd_get_cdtextfile(cd: *const CdPointer) -> *mut libc::c_char;

    pub fn cd_get_ntrack(cd: *const CdPointer) -> libc::c_int;

    pub fn cd_get_cdtext(cd: *const CdPointer) -> *mut CdtextPointer;
    pub fn track_get_cdtext(cd: *const TrackPointer) -> *mut CdtextPointer;
    pub fn cdtext_get(pti: PTI, cdtext: *const CdtextPointer) -> *mut libc::c_char;

    pub fn cd_get_rem(cd: *const CdPointer) -> *mut RemPointer;
    pub fn track_get_rem(track: *const TrackPointer) -> *mut RemPointer;

    pub fn rem_get(index: libc::c_uint, rem: *mut RemPointer) -> *mut libc::c_char;

    pub fn cd_get_track(cd: *const CdPointer, index: libc::c_int) -> *mut TrackPointer;
    pub fn track_get_filename(track: *const TrackPointer) -> *mut libc::c_char;
    pub fn track_get_start(track: *const TrackPointer) -> libc::c_long;
    pub fn track_get_length(track: *const TrackPointer) -> libc::c_long;
    pub fn track_get_mode(track: *const TrackPointer) -> TrackMode;
    pub fn track_get_sub_mode(track: *const TrackPointer) -> TrackSubMode;
    pub fn track_is_set_flag(track: *const TrackPointer, flag: TrackFlag) -> libc::c_int;
    pub fn track_get_zero_pre(track: *const TrackPointer) -> libc::c_long;
    pub fn track_get_zero_post(track: *const TrackPointer) -> libc::c_long;
    pub fn track_get_isrc(track: *const TrackPointer) -> *mut libc::c_char;
    pub fn track_get_index(track: *const TrackPointer, index: libc::c_int) -> libc::c_long;
}
