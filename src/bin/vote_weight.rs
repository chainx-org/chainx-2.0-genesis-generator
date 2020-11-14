// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;

use anyhow::Result;
use chainx_state_exporter::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccountWithVoteWeightInfo {
    account: AccountId,
    nodes: Vec<AccountVoteWeightInfoV1>,
}

impl PartialEq for AccountWithVoteWeightInfo {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

impl Eq for AccountWithVoteWeightInfo {}

impl PartialOrd for AccountWithVoteWeightInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.account.partial_cmp(&other.account)
    }
}

impl Ord for AccountWithVoteWeightInfo {
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
    let height = conf.height;
    let hash = chainx.block_hash(Some(height)).await?;
    log::info!("Block Height {}, Hash: {:?}", height, hash);

    let vote_weight_nodes = if state_exists(height, "vote-weight-nodes.json")? {
        log::info!("Note Vote Weight Info {} already got", height);
        load_state(height, "vote-weight-nodes.json")?
    } else {
        let vote_weight_nodes = chainx
            .total_nodes_vote_weight_v1(hash, height)
            .await?
            .unwrap();
        save_state(height, "vote-weight-nodes.json", &vote_weight_nodes)?;
        vote_weight_nodes
    };
    log::info!("Total Node Number: {}", vote_weight_nodes.len());

    // Each connection handles 1/40 of the total accounts, and 40 connections are required
    const CONNECTION_NUM: usize = 40;

    let mut skip_done = 0;
    for id in 0..account_number / CONNECTION_NUM {
        if state_exists(height, format!("vote-weight-accounts-{}.json", id))? {
            skip_done += 1;
            log::info!("Account Vote Weight Info {}-{} already got", height, id);
        } else {
            break;
        }
    }

    let mut handles = vec![];
    for (id, accounts) in accounts
        .chunks(account_number / CONNECTION_NUM)
        .enumerate()
        .skip(skip_done)
    {
        let chainx = ChainX::new(&conf.chainx_ws_url).await?;
        let accounts = accounts.iter().cloned().collect::<Vec<_>>();
        let handle = async_std::task::spawn(async move {
            chainx
                .total_accounts_vote_weight_v1(id, accounts, hash, height)
                .await
        });
        handles.push((id, handle));
    }

    let mut vote_weight_accounts = vec![];
    for id in 0..skip_done {
        let vote_weight_account: Vec<AccountWithVoteWeightInfo> =
            load_state(height, format!("vote-weight-accounts-{}.json", id))?;
        vote_weight_accounts.extend(vote_weight_account);
    }
    for (id, handle) in handles {
        let info = handle.await?;
        let mut vote_weight_account = info
            .into_iter()
            .map(|(account, nodes)| AccountWithVoteWeightInfo { account, nodes })
            .collect::<Vec<_>>();
        log::info!(
            "[{}] Connection Finished, Account Vote Weight Info Number: {}",
            id,
            vote_weight_account.len()
        );

        vote_weight_account.sort_unstable();
        save_state(
            height,
            format!("vote-weight-accounts-{}.json", id),
            &vote_weight_account,
        )?;
        vote_weight_accounts.extend(vote_weight_account);
    }

    vote_weight_accounts.sort_unstable();
    save_state(height, "vote-weight-accounts.json", &vote_weight_accounts)?;
    log::info!(
        "Total Account Vote Weight Info Number: {}",
        vote_weight_accounts.len()
    );

    // verification
    /*
    let vote_weight_accounts: Vec<AccountWithVoteWeightInfo> =
        load_state(height, "vote-weight-accounts.json")?;
    */

    let total_node_vote_weight = vote_weight_nodes
        .iter()
        .map(|vote_weight_node| vote_weight_node.weight)
        .sum::<u128>();
    let total_account_vote_weight = vote_weight_accounts
        .iter()
        .map(|vote_weight_account| {
            vote_weight_account
                .nodes
                .iter()
                .map(|info| info.node_vote_weight.weight)
                .sum::<u128>()
        })
        .sum::<u128>();
    println!(
        "Total Node Vote Weight: {}, Total Account Vote Weight: {}",
        total_node_vote_weight, total_account_vote_weight
    );
    assert_eq!(total_node_vote_weight, total_account_vote_weight);

    let total_node_vote_nomination = vote_weight_nodes
        .iter()
        .map(|vote_weight_node| vote_weight_node.nomination)
        .sum::<u64>();
    let total_account_vote_nomination = vote_weight_accounts
        .iter()
        .map(|vote_weight_account| {
            vote_weight_account
                .nodes
                .iter()
                .map(|info| info.node_vote_weight.nomination)
                .sum::<u64>()
        })
        .sum::<u64>();
    println!(
        "Total Node Vote Nomination: {}, Total Account Vote Nomination: {}",
        total_node_vote_nomination, total_account_vote_nomination
    );
    assert_eq!(total_node_vote_nomination, total_account_vote_nomination);

    let pcx_staking = chainx
        .assets(0, 10, hash)
        .await?
        .unwrap()
        .data
        .into_iter()
        .filter(|asset| asset.name == "PCX")
        .map(|asset| *asset.details.get(&AssetType::ReservedStaking).unwrap())
        .sum::<u64>();
    assert_eq!(total_node_vote_nomination, pcx_staking);

    Ok(())
}
