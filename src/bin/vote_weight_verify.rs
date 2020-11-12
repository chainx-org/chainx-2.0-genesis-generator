// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;
use std::collections::HashMap;

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

fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;
    let height = conf.height;

    let vote_weight_accounts: Vec<AccountWithVoteWeightInfo> =
        load_state(height, "vote-weight-accounts.json")?;
    let mut nodes1 = HashMap::<AccountId, (u64, u128)>::new();
    for info in vote_weight_accounts {
        for node in info.nodes {
            let node_account = node.node_vote_weight.account;
            let node_nomination = node.node_vote_weight.nomination;
            let node_weight = node.node_vote_weight.weight;
            if node_weight != 0 {
                nodes1
                    .entry(node_account)
                    .and_modify(|value| {
                        (*value).0 += node_nomination;
                        (*value).1 += node_weight;
                    })
                    .or_insert((node_nomination, node_weight));
            }
        }
    }
    let mut nodes1 = nodes1
        .into_iter()
        .map(|node| NodeVoteWeightInfoV1 {
            account: node.0,
            nomination: (node.1).0,
            weight: (node.1).1,
        })
        .collect::<Vec<_>>();
    nodes1.sort_unstable();

    let mut vote_weight_nodes: Vec<NodeVoteWeightInfoV1> =
        load_state(height, "vote-weight-nodes.json")?;
    vote_weight_nodes.sort_unstable();
    let nodes2 = vote_weight_nodes;

    println!("{}, {}", nodes1.len(), nodes2.len());

    for (lhs, rhs) in nodes1.iter().zip(nodes2.iter()) {
        if lhs.account == rhs.account {
            if lhs.nomination == rhs.nomination && lhs.weight == rhs.weight {
                println!("[PASS] node: {:?}", lhs.account);
            } else {
                println!(
                    "[ERROR] node: {:?}, nomination {} | {}, weight {} | {}",
                    lhs.account, lhs.nomination, rhs.nomination, lhs.weight, rhs.weight,
                );
            }
        } else {
            println!(
                "[ERROR] node {:?} | {:?} not match",
                lhs.account, rhs.account
            );
        }
    }

    Ok(())
}
