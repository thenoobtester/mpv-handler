use serde::Deserialize;
use std::path::PathBuf;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_mpv_path")]
    mpv: String,
    #[serde(default = "default_ytdl_path")]
    ytdl: String,
}

impl Config {
    /// Parse the config file
    pub fn parse() -> Result<Self> {
        let path = get_path("config.toml")?;

        let data: Vec<u8> = match std::fs::read(&path) {
            Ok(data) => data,
            Err(_) => return Err(Error::ConfigFailedRead),
        };

        match toml::from_slice(&data) {
            Ok(config) => Ok(config),
            Err(_) => Err(Error::ConfigFailedDeserializeToml),
        }
    }
}

/// Return the config path per OS
fn get_path(file: &str) -> Result<PathBuf> {
    let mut path: PathBuf;

    #[cfg(unix)]
    {
        path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(Error::ConfigFailedGetConfigDir),
        };
        path.push("mpv-handler");
        path.push(file);

        if !path.exists() {
            path = PathBuf::from("/etc/mpv-handler/");
            path.push(file);
        }
    }

    #[cfg(windows)]
    {
        path = std::env::current_exe()?;
        path.pop();
        path.push(file);
    }

    Ok(path)
}

/// Return the default mpv binary path
fn default_mpv_path() -> String {
    #[cfg(unix)]
    {
        String::from("mpv")
    }

    #[cfg(windows)]
    {
        String::from("mpv.com")
    }
}

/// Return the default ytdl binary path (Default: yt-dlp)
fn default_ytdl_path() -> String {
    #[cfg(unix)]
    {
        String::from("yt-dlp")
    }

    #[cfg(windows)]
    {
        String::from("yt-dlp.exe")
    }
}
