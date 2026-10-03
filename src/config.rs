use std::{
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::args::Args;

pub const DEFAULT_LISTEN_PORT: u16 = 5000;
pub const DEFAULT_KEY_BITS: u32 = 1024;
pub const DEFAULT_KEYS_PATH: &str = "keys.json";
pub const DEFAULT_TRACE_CRYPTO: bool = false;
pub const DEFAULT_CONFIG_FILE: &str = "rsachat.toml";
pub const MIN_KEY_BIITS: u32 = 16; 

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub listen_port: u16,
    pub peer: Vec<SocektAddr>,
    pub key_bits: u32,
    pub key_path: PathBuf,
    pub trace_crypto: bool,
} 

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_port: DEFAULT_LISTEN_PORT,
            peer: Vec::new(),
            key_bits: DEFAULT_KEY_BITS,
            key_path: DEFAULT_KEYS_PATH,
            trace_crypto: DEFAULT_TRACE_CRYPTO,
        }
    }
}

impl Config {
    pub fn load(args: &Args) -> Result<Self> {}
    fn from_file(path: &Path)-> Result<Self> {}
    fn apply_cli(&mut self, args: &Args) {}
    fn validate(&self) -> Result<()> {}
}