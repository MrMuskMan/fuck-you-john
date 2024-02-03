use std::{
    ffi::{OsStr, OsString},
    io::{Error, ErrorKind},
    ptr::null_mut,
};

use windows::{
    core::{Result, GUID, HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::{HANDLE, WIN32_ERROR},
        NetworkManagement::WiFi::{
            WlanCloseHandle, WlanEnumInterfaces, WlanFreeMemory, WlanGetProfile,
            WlanGetProfileList, WlanOpenHandle, WLAN_INTERFACE_INFO_LIST, WLAN_PROFILE_INFO_LIST,
        },
    },
};

use crate::AnyResult;

const INVALID_HANDLE: HANDLE = HANDLE(-1);

pub fn open_wlan_handle() -> Result<HANDLE> {
    let mut handle = INVALID_HANDLE;
    WIN32_ERROR(unsafe { WlanOpenHandle(2, None, &mut 0, &mut handle) }).ok()?;
    Ok(handle)
}

pub fn close_wlan_handle(handle: HANDLE) -> Result<()> {
    WIN32_ERROR(unsafe { WlanCloseHandle(handle, None) }).ok()
}

pub fn enum_wlan_interfaces<'l>(handle: HANDLE) -> AnyResult<&'l WLAN_INTERFACE_INFO_LIST> {
    let mut interfaces = null_mut();
    unsafe {
        WIN32_ERROR(WlanEnumInterfaces(handle, None, &mut interfaces)).ok()?;
        interfaces
            .as_ref()
            .ok_or(Box::new(Error::new(ErrorKind::Other, "null pointer")))
    }
}

pub fn enum_interface_profiles<'l>(
    handle: HANDLE,
    interface: GUID,
) -> AnyResult<&'l WLAN_PROFILE_INFO_LIST> {
    let mut profiles = null_mut();
    unsafe {
        WIN32_ERROR(WlanGetProfileList(handle, &interface, None, &mut profiles)).ok()?;
        profiles
            .as_ref()
            .ok_or(Box::new(Error::new(ErrorKind::Other, "null pointer")))
    }
}

pub fn get_profile_xml(
    handle: HANDLE,
    interface: &GUID,
    profile: &OsStr,
    flags: Option<*mut u32>,
) -> Result<OsString> {
    let mut xml = PWSTR::null();

    unsafe {
        WIN32_ERROR(WlanGetProfile(
            handle,
            interface,
            PCWSTR(HSTRING::from(profile).as_ptr()),
            None,
            &mut xml,
            flags,
            None,
        ))
        .ok()?;
        xml.to_hstring()
            .map(|str| str.to_os_string())
            .map_err(|err| {
                WlanFreeMemory(xml.as_ptr().cast());
                err
            })
    }
}
