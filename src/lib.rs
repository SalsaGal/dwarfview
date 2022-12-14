pub mod math;

use std::io::Read;

use xml::{reader::XmlEvent, EventReader};

pub struct Region {
    pub name: String,
    pub altname: String,
}

impl<T> From<T> for Region
where
    T: Read,
{
    fn from(reader: T) -> Self {
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
                    if tags[0] == "df_world" {
                        match &*tags[1] {
                            "name" => name = Some(s),
                            "altname" => altname = Some(s),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Self {
            name: name.unwrap(),
            altname: altname.unwrap(),
        }
    }
}
