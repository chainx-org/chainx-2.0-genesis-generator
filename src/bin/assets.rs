// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;

use anyhow::Result;
use chainx_state_exporter::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccountWithAssets {
    account: AccountId,
    assets: Vec<AssetInfo>,
}

impl PartialEq for AccountWithAssets {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

impl Eq for AccountWithAssets {}

impl PartialOrd for AccountWithAssets {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.account.partial_cmp(&other.account)
    }
}

impl Ord for AccountWithAssets {
    fn cmp(&self, other: &Self) -> Ordering {
        self.account.cmp(&other.account)
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let height = conf.height;

    let accounts: Vec<AccountId> = load_state(height, "accounts.json")?;
    let account_number = accounts.len();
    log::info!("Total Account Number: {}", account_number);

    let chainx = ChainX::new(&conf.chainx_ws_url).await?;
    let hash = chainx.block_hash(Some(height)).await?;
    log::info!("Block Height {}, Hash: {:?}", height, hash);

    if let Some(total_assets) = chainx.assets(0, 10, hash).await? {
        log::info!("Total Assets Info: {:?}", total_assets.data);
        save_state(height, "assets-total.json", &total_assets.data)?;
    }

    // Each connection handles 1/40 of the total accounts, and 40 connections are required
    const CONNECTION_NUM: usize = 40;
    let mut handles = vec![];
    for (id, accounts) in accounts.chunks(account_number / CONNECTION_NUM).enumerate() {
        let chainx = ChainX::new(&conf.chainx_ws_url).await?;
        let accounts = accounts.iter().cloned().collect::<Vec<_>>();
        let handle =
            async_std::task::spawn(
                async move { chainx.total_account_assets(id, accounts, hash).await },
            );
        handles.push((id, handle));
    }

    let mut assets_accounts = vec![];
    for (id, handle) in handles {
        let info = handle.await?;
        let mut assets_account = info
            .into_iter()
            .map(|(account, assets)| AccountWithAssets { account, assets })
            .collect::<Vec<_>>();
        log::info!(
            "[{}] Connection Finished, Account Assets Info Number: {}",
            id,
            assets_account.len()
        );

        assets_account.sort_unstable();
        assets_accounts.extend(assets_account);
    }

    assets_accounts.sort_unstable();
    save_state(height, "assets.json", &assets_accounts)?;
    log::info!(
        "Total Account Assets Info Number: {}",
        assets_accounts.len()
    );

    Ok(())
}
