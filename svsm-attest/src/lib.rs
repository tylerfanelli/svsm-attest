// SPDX-License-Identifier: Apache-2.0

pub mod error;
pub mod proxy;

pub use error::*;

extern crate alloc;

use proxy::SvsmProxyRead;

use alloc::string::String;

use kbs_types::Tee;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Encapsulates all attestation data inputted to the proxy for eventual sending to a remote
/// attestation server. SVSM is responsible for fetching its attestation evidence from the
/// secure processor and creating the private key for encrypted attestation resources.
///
/// This defines a standard for what will be transferred over the serial port from SVSM to the
/// attestation proxy.
#[derive(Debug, Deserialize, Serialize)]
pub struct SvsmProxyInput {
    /// TEE architecture that the evidence should be interpreted from.
    pub tee: Tee,

    /// TEE evidence (i.e. attestation report).
    pub evidence: Value,

    /// PEM-encoded RSA public key.
    pub pubkey_pem: String,
}

impl SvsmProxyInput {
    pub fn from_proxy(proxy: &mut impl SvsmProxyRead) -> Result<SvsmProxyInput> {
        let mut len = [0u8; 4];
        proxy.read_exact(&mut len)?;

        let len: usize = match u32::from_ne_bytes(len).try_into() {
            Ok(l) => l,
            Err(_) => return Err(Error::InputLenDeserialize),
        };

        let mut buf = vec![0u8; len];

        proxy.read_exact(&mut buf)?;

        serde_json::from_slice(&buf).map_err(Error::JsonDeserialize)
    }
}
