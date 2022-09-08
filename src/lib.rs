use std::io::Read;

use xml::{EventReader, reader::XmlEvent};

pub struct Region {
    pub name: String,
    pub altname: String,
}

impl Region {
    pub fn from_file<T: Read>(reader: T) -> Self {
        let parser = EventReader::new(reader);
        let mut tags = Vec::new();

        let mut name = None;
        let mut altname = None;

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    tags.push(name.to_string());
                }
                Ok(XmlEvent::EndElement { name }) => {
                    assert_eq!(*tags.last().unwrap(), name.to_string());
                    tags.pop();
                }
                Ok(XmlEvent::Characters(s)) => {
                    if tags == vec!["df_world", "name"] {
                        name = Some(s);
                    } else if tags == vec!["df_world", "altname"] {
                        altname = Some(s);
                    }
                }
                _ => {},
            }
        }

        Self {
            name: name.unwrap(),
            altname: altname.unwrap(),
        }
    }
}
