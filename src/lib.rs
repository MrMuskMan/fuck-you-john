use std::{error::Error, ffi::OsString, os::windows::ffi::OsStringExt};
pub mod safe_windows;
pub mod wlan;
pub mod xml;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn os_string_from_utf16_slice(slice: &[u16]) -> Option<OsString> {
    Some(OsString::from_wide(
        &slice[..slice.iter().position(|&c| c == 0)?],
    ))
}
