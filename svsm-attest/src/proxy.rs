// SPDX-License-Identifier: Apache-2.0

use crate::error::{Error, Result};

use alloc::boxed::Box;

/// Although from a userspace perspective the proxy is a UNIX socket; an SVSM guest can talk to the
/// socket through a number of interfaces such as a serial device, virtio-vhost socket, etc.
/// SvsmProxyRead allows any of these interfaces to implement their own distinctive way of reading
/// attestation data.
pub trait SvsmProxyRead {
    /// Read bytes from a data stream. Return the number of bytes read from the stream.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read the exact number of bytes from a data stream that are required to fill a buffer of an
    /// arbitrary size.
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        let mut read = 0;
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                    read += n;
                }
                Err(e) => return Err(Error::ProxyRead(Box::new(e))),
            }
        }

        if !buf.is_empty() {
            if read == 0 {
                return Err(Error::ProxyNoDataRead);
            } else {
                return Err(Error::ProxyFillBuffer);
            }
        }

        Ok(())
    }
}
