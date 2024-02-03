use std::{
    ffi::{OsStr, OsString},
    io::{Error, ErrorKind},
    ptr::slice_from_raw_parts,
};

use windows::{
    core::{Result, GUID},
    Win32::Foundation::HANDLE,
};

use crate::{
    os_string_from_utf16_slice,
    safe_windows::{enum_interface_profiles, enum_wlan_interfaces, get_profile_xml},
    xml::Xml,
    AnyResult,
};

use super::safe_windows::{close_wlan_handle, open_wlan_handle};

#[derive(Debug)]
pub enum Authentication {
    Open,
    WPA2(AnyResult<String>),
    WPA2PSK(AnyResult<String>),
    Other,
}

pub struct Wlan {
    handle: HANDLE,
}

impl Wlan {
    pub fn new() -> Result<Self> {
        Ok(Self {
            handle: open_wlan_handle()?,
        })
    }

    pub fn get_interfaces(&self) -> AnyResult<Vec<GUID>> {
        let interfaces = enum_wlan_interfaces(self.handle)?;
        Ok(unsafe {
            (*slice_from_raw_parts(
                interfaces.InterfaceInfo.as_ptr(),
                interfaces.dwNumberOfItems as usize,
            ))
            .iter()
            .map(|info| info.InterfaceGuid)
            .collect::<Vec<GUID>>()
        })
    }

    pub fn get_profiles(&self, interface: GUID) -> AnyResult<Vec<OsString>> {
        let profiles = enum_interface_profiles(self.handle, interface)?;
        Ok(unsafe {
            (*slice_from_raw_parts(
                profiles.ProfileInfo.as_ptr(),
                profiles.dwNumberOfItems as usize,
            ))
            .iter()
            .filter_map(|info| os_string_from_utf16_slice(&info.strProfileName))
            .collect::<Vec<OsString>>()
        })
    }

    pub fn get_authentication(
        &self,
        interface: &GUID,
        profile: &OsStr,
    ) -> AnyResult<Authentication> {
        let xml = Xml::try_from(get_profile_xml(
            self.handle,
            interface,
            profile,
            Some(&mut 4u32),
        )?)?;
        let authentication = xml
            .get(&["MSM", "security", "authEncryption", "authentication"])
            .ok_or(Box::new(Error::new(
                ErrorKind::Other,
                "couldn't get authentication",
            )))?;
        Ok(match authentication.as_str() {
            "open" => Authentication::Open,
            "WPA2" => {
                if let Some(password) = xml.get(&["MSM", "security", "sharedKey", "keyMaterial"]) {
                    Authentication::WPA2(Ok(password))
                } else {
                    Authentication::WPA2(Err(Box::new(Error::new(
                        ErrorKind::Other,
                        "couldn't get password",
                    ))))
                }
            }
            "WPA2PSK" => {
                if let Some(password) = xml.get(&["MSM", "security", "sharedKey", "keyMaterial"]) {
                    Authentication::WPA2PSK(Ok(password))
                } else {
                    Authentication::WPA2PSK(Err(Box::new(Error::new(
                        ErrorKind::Other,
                        "couldn't get password",
                    ))))
                }
            }
            _ => Authentication::Other,
        })
    }
}

impl Drop for Wlan {
    fn drop(&mut self) {
        close_wlan_handle(self.handle).expect("couldn't close wlan handle");
    }
}
