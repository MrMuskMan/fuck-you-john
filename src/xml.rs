use std::{error::Error, ffi::OsString, os::windows::ffi::OsStringExt};

use windows::{core::HSTRING, Data::Xml::Dom::XmlDocument};

use crate::AnyResult;

pub struct Xml {
    document: XmlDocument,
}

impl Xml {
    pub fn get<'l>(&self, path: &'l [&'l str]) -> Option<String> {
        let mut subtree = self.document.DocumentElement().ok()?.ChildNodes().ok()?;
        let &last = path.last()?;
        'traverse: for &node in path {
            let name = OsString::from_wide(&node.encode_utf16().collect::<Vec<u16>>());

            for value in &subtree {
                if let Ok(_name) = value.NodeName() {
                    if _name.to_os_string() == name {
                        if name.to_string_lossy() == last {
                            return Some(value.InnerText().ok()?.to_string());
                        }

                        subtree = value.ChildNodes().ok()?;
                        continue 'traverse;
                    }
                }
            }
        }
        None
    }
}

impl Default for Xml {
    fn default() -> Xml {
        Xml {
            document: XmlDocument::new().expect("couldn't make document"),
        }
    }
}

impl TryFrom<OsString> for Xml {
    type Error = Box<dyn Error>;
    fn try_from(os_str: OsString) -> AnyResult<Self> {
        let xml = Self::default();
        xml.document.LoadXml(&HSTRING::from(os_str))?;
        Ok(xml)
    }
}
