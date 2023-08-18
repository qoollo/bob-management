use clap::Parser;
use color_eyre::{eyre::eyre, Report, Result};
use std::path::PathBuf;

use crate::config::Config;

/// Bob configuration
#[derive(Debug, Parser, Clone)]
#[command(author = "Romanov Simeon <ArchArcheoss@proton.me>")]
#[command(version, about, long_about)]
#[group(id = "configs", required = true, multiple = false)]
pub struct Args {
    /// If set, passes default configuration to the server
    #[clap(short, long)]
    default: bool,

    /// Server configuration file
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,
}

impl TryFrom<Args> for Config {
    type Error = Report;

    fn try_from(value: Args) -> Result<Self> {
        if value.default {
            Ok(Self::default())
        } else if let Some(config) = value.config_file {
            Self::from_file(config)
        } else {
            Err(eyre!("Unexpected error: empty configuration"))
        }
    }
}
