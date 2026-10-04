use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub fn run(config: &Config) -> Result<()> {
    info!(target: "server", "listening on port {}", config.listen_port); 
    info!(target: "server", "peers to contact: {:?}", config.peers); 
    info!(target: "crypto", "{}-bit keys → {}", config.key_bits, config.keys_path.display()); 
    info!(target: "client", "interface ready (crypto tracing: {})", config.trace_crypto);
    Ok(())
}
