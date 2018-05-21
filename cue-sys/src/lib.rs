extern crate libc;

pub enum CdPointer {}
pub enum CdtextPointer {}
pub enum TrackPointer {}
pub enum RemPointer {}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum DiscMode {
    CD_DA,
    CD_ROM,
    CD_ROM_XA,
}

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

#[repr(C)]
pub enum TrackMode {
    Audio,
    /// 2048-byte data without ECC
    Mode1,
    /// 2048-byte data with ECC
    Mode1Raw,
    /// 2336-byte data without ECC
    Mode2,
    /// 2048-byte data (CD-ROM XA)
    Mode2Form1,
    /// 2324-byte data (CD-ROM XA)
    Mode2Form2,
    /// 2332-byte data (CD-ROM XA)
    Mode2FormMix,
    /// 2336-byte data with ECC
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
    None = 0x00,
    PreEmphasis = 0x01,
    CopyPermitted = 0x02,
    Data = 0x04,
    FourChannel = 0x08,
    SCMS = 0x10,
    Any = 0xFF,
}

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
