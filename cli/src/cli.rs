use clap::{crate_authors, crate_version, Parser};
use color_eyre::{eyre::eyre, Report, Result};
use std::path::PathBuf;

use crate::config::{Config, FromFile, LoggerConfig};
const VERSION: &str = concat!(
    "BOB-GUI VERSION: ",
    crate_version!(),
    "\n",
    "BUILT AT: ",
    env!("BUILD_TIME"),
    "\n",
    "COMMIT HASH: ",
    env!("GIT_HASH"),
    "\n",
    "GIT BRANCH: ",
    env!("GIT_BRANCH"),
    "\n",
    "GIT TAG: ",
    env!("GIT_TAG"),
);

/// Bob configuration
#[derive(Debug, Parser, Clone)]
#[command(author = crate_authors!())]
#[command(version = VERSION, about, long_about)]
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

impl TryFrom<Args> for LoggerConfig {
    type Error = Report;

    fn try_from(value: Args) -> Result<Self> {
        if value.default {
            Ok(Self::default())
        } else if let Some(config) = value.config_file {
            Self::from_file(config)
        } else {
            Err(eyre!("Unexpected error: empty logger configuration"))
        }
    }
}
