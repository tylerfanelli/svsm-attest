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
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProxyRead(e) => write!(f, "unable to read buffer: {}", e),
            Self::ProxyNoDataRead => write!(f, "no data read from buffer"),
            Self::ProxyFillBuffer => write!(f, "unable to fill buffer"),

            #[cfg(feature = "std")]
            Self::UnixSocketRead(io) => write!(f, "unable to read from unix socket: {}", io),
        }
    }
}
