// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;

use anyhow::Result;
use chainx_state_exporter::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccountWithDepositWeightInfo {
    account: AccountId,
    #[serde(flatten)]
    deposit_weight: TotalDepositWeightInfoV1,
}

impl PartialEq for AccountWithDepositWeightInfo {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

impl Eq for AccountWithDepositWeightInfo {}

impl PartialOrd for AccountWithDepositWeightInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.account.partial_cmp(&other.account)
    }
}

impl Ord for AccountWithDepositWeightInfo {
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

    // let total_node_deposit_weight = chainx
    //     .total_node_deposit_weight_v1(hash, height)
    //     .await?
    //     .unwrap();
    let total_node_deposit_weight = chainx
        .total_node_raw_deposit_weight_v1(hash, height)
        .await?
        .unwrap();
    save_state(
        height,
        "deposit-weight-nodes.json",
        &total_node_deposit_weight,
    )?;
    log::info!(
        "Total node deposit weight: X-BTC [{:?}], L-BTC [{:?}], S-DOT [{:?}]",
        total_node_deposit_weight.xbtc,
        total_node_deposit_weight.lbtc,
        total_node_deposit_weight.sdot
    );

    // Each connection handles 1/40 of the total accounts, and 40 connections are required
    const CONNECTION_NUM: usize = 40;
    let mut handles = vec![];
    for (id, accounts) in accounts.chunks(account_number / CONNECTION_NUM).enumerate() {
        let chainx = ChainX::new(&conf.chainx_ws_url).await?;
        let accounts = accounts.iter().cloned().collect::<Vec<_>>();
        let handle = async_std::task::spawn(async move {
            chainx
                .total_accounts_deposit_weight_v1(id, accounts, hash, height)
                .await
        });
        handles.push((id, handle));
    }

    let mut deposit_weight_accounts = vec![];
    for (id, handle) in handles {
        let info = handle.await?;
        let mut deposit_weight_account = info
            .into_iter()
            .map(|(account, deposit_weight)| AccountWithDepositWeightInfo {
                account,
                deposit_weight,
            })
            .collect::<Vec<_>>();
        log::info!(
            "[{}] Connection Finished, Account Deposit Weight Info Number: {}",
            id,
            deposit_weight_account.len()
        );

        deposit_weight_account.sort_unstable();
        deposit_weight_accounts.extend(deposit_weight_account);
    }

    deposit_weight_accounts.sort_unstable();
    save_state(
        height,
        "deposit-weight-accounts.json",
        &deposit_weight_accounts,
    )?;
    log::info!(
        "Total Account Deposit Weight Info Number: {}",
        deposit_weight_accounts.len()
    );

    // verification
    /*
    let deposit_weight_accounts: Vec<AccountWithDepositWeightInfo> =
        load_state(height, "deposit-weight-accounts.json")?;
    */

    let account_xbtc_deposit_weight = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.xbtc.weight)
        .sum::<u128>();
    println!(
        "Total Node XBTC Deposit Weight: {}, Total Account XBTC Deposit Weight: {}",
        total_node_deposit_weight.xbtc.weight, account_xbtc_deposit_weight
    );
    let account_xbtc_balance = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.xbtc.balance)
        .sum::<Balance>();
    assert_eq!(total_node_deposit_weight.xbtc.balance, account_xbtc_balance);

    let account_lbtc_deposit_weight = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.lbtc.weight)
        .sum::<u128>();
    println!(
        "Total Node LBTC Deposit Weight: {}, Total Account LBTC Deposit Weight: {}",
        total_node_deposit_weight.lbtc.weight, account_lbtc_deposit_weight
    );
    let account_lbtc_balance = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.lbtc.balance)
        .sum::<Balance>();
    assert_eq!(total_node_deposit_weight.lbtc.balance, account_lbtc_balance);

    let account_sdot_deposit_weight = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.sdot.weight)
        .sum::<u128>();
    println!(
        "Total Node SDOT Deposit Weight: {}, Total Account SDOT Deposit Weight: {}",
        total_node_deposit_weight.sdot.weight, account_sdot_deposit_weight
    );
    let account_sdot_balance = deposit_weight_accounts
        .iter()
        .map(|deposit_weight| deposit_weight.deposit_weight.sdot.balance)
        .sum::<Balance>();
    assert_eq!(total_node_deposit_weight.sdot.balance, account_sdot_balance);
    Ok(())
}
