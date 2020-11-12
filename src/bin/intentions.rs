// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use anyhow::Result;
use chainx_state_exporter::*;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let height = conf.height;

    let chainx = ChainX::new(&conf.chainx_ws_url).await?;
    let hash = chainx.block_hash(Some(height)).await?;
    log::info!("Block Height {}, Hash: {:?}", height, hash);

    let mut intentions = chainx
        .intentions_v1(hash)
        .await?
        .expect("intentions must exist; qed");

    intentions.sort_unstable();
    save_state(height, "intentions.json", &intentions)?;
    log::info!("Total Intentions Number: {}", intentions.len());

    Ok(())
}
