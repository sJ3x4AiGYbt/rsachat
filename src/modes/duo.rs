use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Context, Result};
use tracing::info;

use crate::config::Config;

pub fn run(config: &Config) -> Result<()> {
    let alice_port = config.listen_port;
    let bob_port = alice_port
        .checked_add(1)
        .context("listen_port too big for duo")?;
    let alice_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, alice_port));

    info!(target: "server", "alice : listening on {alice_addr}");
    info!(target: "server", "bob   : listening on port {bob_port}, connects to {alice_addr}");
    info!(target: "client", "two side-by-side interfaces (coming with ratatui)");
    Ok(())
}
