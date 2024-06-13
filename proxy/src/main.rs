// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use anyhow::Context;
use clap::Parser;
use log::error;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// HTTP URL to remote attestation server.
    #[clap(long)]
    url: String,

    /// Path in which the unix domain socket will be created.
    #[clap(long)]
    unix: String,

    /// Force deletion of unix socket if path already exists.
    force: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    if args.force {
        let _ = fs::remove_file(args.unix.clone());
    }

    let listener = UnixListener::bind(args.unix).context("unable to listen on unix socket")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let url = args.url.clone();
                thread::spawn(|| proxy(stream, url));
            }

            Err(e) => {
                error!("{}", e);
            }
        }
    }

    Ok(())
}

fn proxy(stream: UnixStream, url: String) {
    todo!();
}