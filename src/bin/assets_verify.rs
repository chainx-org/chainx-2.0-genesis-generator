// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::collections::BTreeMap;

use anyhow::Result;
use chainx_state_exporter::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccountWithAssets {
    account: AccountId,
    assets: Vec<AssetInfo>,
}

fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let height = conf.height;

    let total_assets: [TotalAssetInfo; 4] = load_state(height, "assets-total.json")?;
    let (total_pcx, total_xbtc, total_lbtc, total_sdot) = (
        total_assets[0].details.clone(),
        total_assets[1].details.clone(),
        total_assets[2].details.clone(),
        total_assets[3].details.clone(),
    );

    let assets_accounts: Vec<AccountWithAssets> = load_state(height, "assets.json")?;

    let mut total_account_pcx = BTreeMap::new();
    let mut total_account_xbtc = BTreeMap::new();
    let mut total_account_lbtc = BTreeMap::new();
    let mut total_account_sdot = BTreeMap::new();
    for account in assets_accounts {
        let assets = account.assets;
        for asset in assets {
            match asset.name.as_str() {
                "PCX" => sum_asset(&mut total_account_pcx, asset.details),
                "BTC" => sum_asset(&mut total_account_xbtc, asset.details),
                "L-BTC" => sum_asset(&mut total_account_lbtc, asset.details),
                "SDOT" => sum_asset(&mut total_account_sdot, asset.details),
                _ => {}
            }
        }
    }

    assert_eq!(total_pcx, total_account_pcx);
    assert_eq!(total_xbtc, total_account_xbtc);
    assert_eq!(total_lbtc, total_account_lbtc);
    assert_eq!(total_sdot, total_account_sdot);

    Ok(())
}

fn sum_asset(sum: &mut BTreeMap<AssetType, Balance>, add: BTreeMap<AssetType, Balance>) {
    for (asset, balance) in add {
        sum.entry(asset)
            .and_modify(|value| *value += balance)
            .or_insert(balance);
    }
}
