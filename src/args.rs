use std::{net::SocketAddr, path::PathBuf};

use clap::{Parser, Subcommand};

/// peer-to-peer encrypted chat over TCP with hand-rolled RSA
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// TOML configuration file (default: ./rsachat.toml if it exists)
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    /// Peer-to-peer listening port
    #[arg(short, long, global = true)]
    pub port: Option<u16>,
    /// Peer to contact at startup (repeatable)
    #[arg(long, global = true)]
    pub peer: Vec<SocketAddr>,
    /// RSA key size in bits
    #[arg(long, global = true)]
    pub key_bits: Option<u32>,
    /// Display each step of the encryption process
    #[arg(long, global = true)]
    pub trace_crypto: bool,
    #[commande(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Run, 
    Duo,
}