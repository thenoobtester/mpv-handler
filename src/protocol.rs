use crate::error::{Error, Result};

const DEFAULT_DOWNLOADER: &str = "ytdl";

#[derive(Debug)]
pub struct Protocol {
    pub cookies: bool,
    pub downloader: String,
    pub quality: i32,
    pub url: String,
}

impl Protocol {
    fn new() -> Self {
        Protocol {
            cookies: false,
            downloader: String::from(DEFAULT_DOWNLOADER),
            quality: 0,
            url: String::new(),
        }
    }

    pub fn parse(arg: &mut String) -> Result<Self> {
        match arg.starts_with("mpv://play/") {
            true => arg.replace_range(0.."mpv://play/".len(), ""),
            false => return Err(Error::ProtocolBadUrl),
        }

        if arg.ends_with('/') {
            arg.pop();
        }

        let mut protocol = Protocol::new();
        let args: Vec<&str> = arg.split("/?").collect();

        match args.get(0) {
            Some(data) => parse_url(&mut protocol, data)?,
            None => return Err(Error::ProtocolVideoUrlNotFound),
        };

        match args.get(1) {
            Some(data) => {
                let options: Vec<&str> = data.split('&').collect();

                for option in options {
                    let option_data: Vec<&str> = option.split('=').collect();

                    let option_name: &str = match option_data.get(0) {
                        Some(name) => *name,
                        None => return Err(Error::ProtocolOptionBadKey),
                    };

                    let option_value: &str = match option_data.get(1) {
                        Some(value) => *value,
                        None => return Err(Error::ProtocolOptionBadValue),
                    };

                    parse_option(&mut protocol, option_name, option_value)?;
                }
            }
            None => {}
        };

        Ok(protocol)
    }
}

/// Get the video url from base64 data
fn parse_url(protocol: &mut Protocol, data: &&str) -> Result<()> {
    match data.len() {
        0 => return Err(Error::ProtocolVideoUrlNotFound),
        _ => protocol.url = String::from_utf8(base64::decode(data)?)?,
    };

    Ok(())
}

/// Get the optional parameters
fn parse_option(protocol: &mut Protocol, option_name: &str, option_value: &str) -> Result<()> {
    match option_name {
        "c" | "cookies" => protocol.cookies = option_value.parse()?,
        "d" | "downloader" => protocol.downloader = option_value.to_string(),
        "q" | "quality" => protocol.quality = option_value.parse()?,
        _ => return Err(Error::ProtocolOptionNotFound),
    };

    Ok(())
}
