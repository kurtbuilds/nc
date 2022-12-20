extern crate core;

use std::fs;
use std::io::Write;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::net;
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    hostname: Option<String>,
    port: Option<String>,

    #[clap(short, long)]
    udp: bool,

    #[clap(short, long)]
    listen: bool,

    #[clap(short = 'w', long)]
    timeout: Option<u32>,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short = 'R', long)]
    replace: Option<String>,
}

fn listen_udp(port: u16, replace: Option<String>) -> Result<()> {
    let socket = net::UdpSocket::bind(format!("127.0.0.1:{}", port))?;
    let mut buf = [0u8; 1024];
    let mut stdout = std::io::stdout().lock();
    loop {
        let n = socket.recv(&mut buf)?;
        let data = &buf[..n];
        if let Some(ref replace) = replace {
            let s = replace.replace("$0", &String::from_utf8_lossy(data));
            stdout.write(s.as_bytes()).unwrap();
            stdout.flush().unwrap();
        } else {
            stdout.write(&data).unwrap();
            stdout.flush().unwrap();
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.listen {
        let port = args.hostname.expect("Must supply a port.");
        let port = port.parse::<u16>().expect("Port must be a number.");
        let replace = args.replace.map(|r| {
            r.replace("\\n", "\n")
        });
        if args.udp {
            listen_udp(port, replace)
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}