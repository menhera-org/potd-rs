
use std::fmt::Display;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Error type for the POTD feed parser
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl From<quick_xml::Error> for ParseError {
    fn from(e: quick_xml::Error) -> Self {
        ParseError {
            message: format!("{:?}", e),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

/// Parse the POTD feed and extract URLs
pub fn parse(atom_source: &str) -> Result<Vec<String>, ParseError> {
    let mut reader = Reader::from_str(atom_source);
    reader.config_mut().trim_text(true);
    let mut urls = Vec::new();

    let mut in_item = false;
    let mut in_description = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = name.as_ref();
                if name == b"item" {
                    in_item = true;
                    log::trace!("Found item");
                } else if in_item && name == b"description" {
                    in_description = true;
                    log::trace!("Found description");
                } else {
                    let name_str = std::str::from_utf8(name).unwrap_or("<invalid>");
                    log::trace!("Found start tag: {:?}", name_str);
                }
            }
            Ok(Event::Text(e)) => {
                if in_description {
                    log::trace!("Found description text");
                    let text = if let Ok(t) = e.unescape() {
                        t.into_owned()
                    } else {
                        continue;
                    };
                    let matches = text.split("src=\"").collect::<Vec<&str>>();
                    if matches.len() < 2 {
                        log::debug!("No src attribute found in description text: {:?}", &text);
                        continue;
                    }
                    let url = matches[1].split("\"").collect::<Vec<&str>>()[0];
                    urls.push(url.to_string());
                }
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                let name = name.as_ref();
                if name == b"item" {
                    in_item = false;
                    log::trace!("End of item");
                } else if name == b"description" {
                    in_description = false;
                    log::trace!("End of description");
                } else {
                    let name_str = std::str::from_utf8(name).unwrap_or("<invalid>");
                    log::trace!("Found end tag: {:?}", name_str);
                }
            }
            Ok(Event::Eof) => {
                log::trace!("End of file");
                break;
            },
            Err(e) => {
                log::error!("Error at position {}: {:?}", reader.buffer_position(), e);
                return Err(ParseError::from(e));
            },
            _ => (),
        }
    }

    Ok(urls)
}
