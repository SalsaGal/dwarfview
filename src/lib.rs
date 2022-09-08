use std::io::Read;

use xml::{EventReader, reader::XmlEvent};

pub struct Region {
    pub name: String,
}

impl Region {
    pub fn from_file<T: Read>(reader: T) -> Self {
        let parser = EventReader::new(reader);
        let mut parsing_state = ParsingState::None;

        let mut name = None;

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match &*name.to_string() {
                        "name" => parsing_state = ParsingState::Name,
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if matches!(parsing_state, ParsingState::Name) {
                        name = Some(s);
                    }
                }
                _ => {},
            }
        }

        Self {
            name: name.unwrap(),
        }
    }
}

enum ParsingState {
    Name,
    None,
}
