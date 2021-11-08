mod config;
mod downloader;
mod error;
mod protocol;

use config::Config;
use downloader::ytdl;
use error::{Error, Result};
use protocol::Protocol;

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    let arg: &mut String = match args.len() {
        2 => args.get_mut(1).unwrap(),
        1 => return Err(Error::MainArgsNone),
        _ => return Err(Error::MainArgsTooMany),
    };

    match arg.as_str() {
        "version" | "-v" | "-V" => version(),
        _ => {}
    };

    let protocol = Protocol::parse(arg)?;
    let config = Config::parse()?;
    let info = match protocol.downloader.as_str() {
        "ytdl" => ytdl::dump_json(),
        _ => return Err(Error::MainDownloaderNotFound),
    };

    dbg!(&protocol);
    dbg!(&config);
    dbg!(&info);

    Ok(())
}

/// Print `mpv-handler` version infomation
fn version() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}", version);

    std::process::exit(0)
}
