// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

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
