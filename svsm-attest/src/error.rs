// SPDX-License-Identifier: Apache-2.0

use core::{
    fmt::{self, Display},
    result,
};

#[cfg(feature = "std")]
use std::io;

use alloc::boxed::Box;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ProxyRead(Box<Self>),
    ProxyNoDataRead,
    ProxyFillBuffer,

    #[cfg(feature = "std")]
    UnixSocketRead(io::Error),

    IoLenSerialization,
    JsonDeserialize(serde_json::Error),

    WriteZero,
    #[cfg(feature = "std")]
    UnixSocketWrite(io::Error),
    #[cfg(feature = "std")]
    UnixSocketFlush(io::Error),

    JsonSerialize(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProxyRead(e) => write!(f, "unable to read buffer: {}", e),
            Self::ProxyNoDataRead => write!(f, "no data read from buffer"),
            Self::ProxyFillBuffer => write!(f, "unable to fill buffer"),

            #[cfg(feature = "std")]
            Self::UnixSocketRead(io) => write!(f, "unable to read from unix socket: {}", io),

            Self::IoLenSerialization => write!(f, "unable to convert input length to u32"),
            Self::JsonDeserialize(e) => {
                write!(f, "unable to deserialize SVSM proxy input from JSON: {}", e)
            }
            Self::WriteZero => write!(f, "wrote zero bytes to proxy"),
            #[cfg(feature = "std")]
            Self::UnixSocketWrite(io) => write!(f, "unable to write to unix socket: {}", io),
            #[cfg(feature = "std")]
            Self::UnixSocketFlush(io) => write!(f, "unable to flush unix socket: {}", io),
            Self::JsonSerialize(json) => write!(f, "unable to serialize to JSON: {}", json),
        }
    }
}
