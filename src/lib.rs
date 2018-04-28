extern crate libc;

#[repr(C)]
pub enum DataType {
    Audio,
    Data,
    FIFO,
    Zero,
}

pub mod cd;
pub mod cd_text;
pub mod rem;
pub mod track;
mod raw;
