// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use chainx_state_exporter::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SessionIndexWithHeight {
    height: BlockNumber,
    session_index: BlockNumber,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let height = conf.height;

    let chainx = ChainX::new(&conf.chainx_ws_url).await?;
    let hash = chainx.block_hash(Some(height)).await?;
    log::info!("Block Height {}, Hash: {:?}", height, hash);

    let session_index = chainx.session_index(hash).await?.unwrap();
    log::info!(
        "Current Session Index Of Height {}: {}",
        height,
        session_index
    );
    let value = SessionIndexWithHeight {
        height,
        session_index,
    };
    save_state(height, "session-index.json", &value)?;

    Ok(())
}
