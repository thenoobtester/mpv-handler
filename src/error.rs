pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Main
    #[error("")]
    MainArgsNone,
    #[error("")]
    MainArgsTooMany,
    #[error("")]
    MainDownloaderNotFound,

    // Protocol
    #[error("")]
    ProtocolBadUrl,
    #[error("")]
    ProtocolVideoUrlNotFound,
    #[error("")]
    ProtocolOptionBadKey,
    #[error("")]
    ProtocolOptionBadValue,
    #[error("")]
    ProtocolOptionNotFound,

    // Config
    #[error("")]
    ConfigFailedRead,
    #[error("")]
    ConfigFailedDeserializeToml,
    #[error("")]
    ConfigFailedGetConfigDir,

    // Downloader

    // Transparent
    #[error(transparent)]
    FailedDecodeBase64(#[from] base64::DecodeError),
    #[error(transparent)]
    FailedDecodeUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    FailedParseBool(#[from] std::str::ParseBoolError),
    #[error(transparent)]
    FailedParseInt(#[from] std::num::ParseIntError),
}
