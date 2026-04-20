//! Shared types, errors, and utilities used across the workspace.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Top-level error type shared across crates.
#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Application configuration shared across crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

/// Initialise tracing with an `EnvFilter` based on `RUST_LOG`.
pub fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_uses_port_3000() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.port, 3000);
        assert_eq!(cfg.host, "0.0.0.0");
    }
}
