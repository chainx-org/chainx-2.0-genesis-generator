// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;
use structopt::StructOpt;
use url::Url;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = "chainx-state-exporter",
    author = "The ChainX Authors <https://chainx.org>",
    about = "Export the required status of ChainX-v1.x to prepare for upgrading to the v2.0"
)]
pub struct CmdConfig {
    #[structopt(short, long, value_name = "FILE", default_value = "config.json")]
    pub config: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// ChainX WebSocket url.
    pub chainx_ws_url: Url,
    /// The block height of ChainX.
    pub height: u64,
}

impl CmdConfig {
    /// Generate config from command.
    pub fn init() -> Result<Config> {
        let cmd: CmdConfig = CmdConfig::from_args();
        let file = File::open(cmd.config)?;
        Ok(serde_json::from_reader(file)?)
    }
}
