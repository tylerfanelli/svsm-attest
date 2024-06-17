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

/// Encapsulates all attestation data outputted from the proxy with results from a remote
/// attestation server. Since data is written over a socket, a boolean indicator of success must be
/// checked to ensure attestation was successful. If attestation was unsuccessful, the encrypted
/// results should not be read.
///
/// If attestation was successful, the encrypted results will be JSON-serialized.
///
/// This defines a standard for what will be transferred from the attestation proxy to SVSM.
#[derive(Debug, Deserialize, Serialize)]
pub struct SvsmProxyOutput {
    /// Indicator of attestation success.
    pub success: bool,

    /// Encrypted attestation results.
    pub res_encrypted: Value,
}
