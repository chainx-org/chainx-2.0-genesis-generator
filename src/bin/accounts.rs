// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use chainx_state_exporter::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct NewAccount {
    height: u64,
    account: AccountId,
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let target_height = conf.height;

    // Save accounts from block #0
    if accounts_exists("genesis.json")? {
        log::info!("Accounts (Block #0) already got");
    } else {
        let chainx = ChainX::new(&conf.chainx_ws_url).await?;
        let accounts = chainx.new_account(0, vec![0]).await?;
        let mut accounts = accounts
            .into_iter()
            .map(|(height, account)| NewAccount { height, account })
            .collect::<Vec<_>>();
        accounts.sort_unstable();
        save_accounts("genesis.json", &accounts)?;
    }

    const CHUNK_NUMBER: u64 = 100_000;

    for height in (1..=target_height).step_by(CHUNK_NUMBER as usize) {
        if accounts_exists(format!("{}-{}.json", height, height + CHUNK_NUMBER - 1))? {
            log::info!(
                "Accounts {}-{} already got",
                height,
                height + CHUNK_NUMBER - 1
            );
            continue;
        }

        // Each connection handles 1/50 of the total blocks, and 50 connections are required
        const CONNECTION_NUM: u64 = 50;
        let chunk_size = CHUNK_NUMBER / CONNECTION_NUM;

        let heights = (height..height + CHUNK_NUMBER).collect::<Vec<_>>();

        let mut handles = vec![];
        for (id, chunk_heights) in heights.chunks(chunk_size as usize).enumerate() {
            let chainx = ChainX::new(&conf.chainx_ws_url).await?;
            let heights = chunk_heights.iter().copied().collect::<Vec<u64>>();
            let handle =
                async_std::task::spawn(async move { chainx.new_account(id, heights).await });
            handles.push((id, handle));
        }

        let mut total_new_accounts = vec![];
        for (id, handle) in handles {
            let accounts = handle.await?;
            let mut new_accounts = accounts
                .into_iter()
                .map(|(height, account)| NewAccount { height, account })
                .collect::<Vec<_>>();
            log::info!(
                "[{}] Connection Finished, New Account Number: {}",
                id,
                new_accounts.len()
            );

            new_accounts.sort_unstable();
            total_new_accounts.extend(new_accounts);
        }

        total_new_accounts.sort_unstable();
        save_accounts(
            format!("{}-{}.json", height, height + CHUNK_NUMBER - 1),
            &total_new_accounts,
        )?;
        log::info!("Total New Account Number: {}", total_new_accounts.len());
    }

    // collect needed accounts for querying other states
    let mut total_accounts = load_accounts::<_, Vec<NewAccount>>("genesis.json")?
        .into_iter()
        .map(|val| val.account)
        .collect::<Vec<_>>();

    for height in (1..=target_height).step_by(CHUNK_NUMBER as usize) {
        let accounts = load_accounts::<_, Vec<NewAccount>>(format!(
            "{}-{}.json",
            height,
            height + CHUNK_NUMBER - 1
        ))?
        .into_iter()
        .map(|val| val.account);
        total_accounts.extend(accounts);
    }

    total_accounts.sort_unstable();
    log::info!("Total Account Number: {}", total_accounts.len());

    save_state(target_height, "accounts.json", &total_accounts)?;

    Ok(())
}
