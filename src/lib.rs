use std::{error::Error, ffi::OsString, os::windows::ffi::OsStringExt, ptr::slice_from_raw_parts};
pub mod safe_windows;
pub mod wlan;
pub mod xml;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn raw_parts_to_slice<'l, T>(raw: *const T, len: usize) -> &'l [T] {
    unsafe { &*slice_from_raw_parts(raw, len) }
}

pub fn os_string_from_utf16_slice(slice: &[u16]) -> Option<OsString> {
    Some(OsString::from_wide(
        &slice[..slice.iter().position(|&c| c == 0)?],
    ))
}
