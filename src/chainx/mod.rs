// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

mod decode;
mod rpc;
mod types;

pub use self::decode::*;
pub use self::types::*;

use crate::log_missing_block_height;
use anyhow::Result;
use codec::Decode;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sp_core::{
    storage::{StorageData, StorageKey},
    twox_128,
};
use url::Url;
use web3::transports::WebSocket;

/// The ChainX Rpc client
#[derive(Clone)]
pub struct ChainX {
    client: WebSocket,
}

impl ChainX {
    /// Creates a new Rpc Client.
    pub async fn new(url: &Url) -> Result<Self> {
        assert!(
            url.as_str().starts_with("ws://") || url.as_str().starts_with("wss://"),
            "the url accepts websocket protocol only"
        );
        let client = WebSocket::new(url.as_str()).await?;
        Ok(Self { client })
    }

    pub async fn new_account(&self, id: usize, heights: Vec<u64>) -> Result<Vec<(u64, AccountId)>> {
        assert!(!heights.is_empty());
        let mut accounts = vec![];
        let (begin, end) = (*heights.first().unwrap(), *heights.last().unwrap());
        for height in heights {
            let storage = self.system_events(height).await?;
            log::info!("[{}] [{}-{}] Block Height {}", id, begin, end, height);
            if let Some(storage) = storage {
                let event_records: Vec<EventRecord<ChainXEvent, Hash>> =
                    match Decode::decode(&mut storage.0.as_slice()) {
                        Ok(records) => records,
                        Err(err) => {
                            log::error!("Block Height {}, err: {}", height, err);
                            log_missing_block_height(height)?;
                            continue;
                        }
                    };
                for event_record in event_records {
                    if let ChainXEvent::XAssets(XAssetsEvent::NewAccount(account)) =
                        event_record.event
                    {
                        accounts.push((height, account))
                    }
                }
            }
        }
        Ok(accounts)
    }

    pub async fn system_events(&self, height: u64) -> Result<Option<StorageData>> {
        let hash = self.block_hash(Some(height)).await?;
        let key = StorageKey(twox_128(b"System Events").to_vec());
        self.storage(&key, hash).await
    }

    pub async fn total_nodes_vote_weight_v1(
        &self,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Option<Vec<NodeVoteWeightInfoV1>>> {
        let intentions = match self.intentions(hash).await {
            Ok(Some(intentions)) => Some(
                intentions
                    .into_iter()
                    .map(|intention| IntentionInfoV1 {
                        intention_common: intention.intention_common,
                        intention_profs: IntentionProfsV1 {
                            total_nomination: intention.intention_profs.total_nomination,
                            last_total_vote_weight: intention
                                .intention_profs
                                .last_total_vote_weight
                                .to_string(),
                            last_total_vote_weight_update: intention
                                .intention_profs
                                .last_total_vote_weight_update,
                        },
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => self.intentions_v1(hash).await?,
        };
        if let Some(intentions) = intentions {
            let nodes_vote_weight = intentions
                .into_iter()
                .map(|intention| NodeVoteWeightInfoV1 {
                    account: intention.intention_common.account,
                    nomination: intention.intention_profs.total_nomination,
                    weight: calc_node_vote_weight_v1(&intention, height),
                })
                .filter(|info| info.weight != 0)
                .collect::<Vec<_>>();
            Ok(Some(nodes_vote_weight))
        } else {
            Ok(None)
        }
    }

    pub async fn total_accounts_vote_weight_v1(
        &self,
        id: usize,
        accounts: Vec<AccountId>,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Vec<(AccountId, Vec<AccountVoteWeightInfoV1>)>> {
        let account_number = accounts.len();
        let mut vote_weight_accounts = vec![];
        for (index, account) in accounts.into_iter().enumerate() {
            let vote_weight = self.account_vote_weight_v1(&account, hash, height).await?;
            if let Some(vote_weight) = vote_weight {
                log::info!(
                    "[{}] ({} / {}) Account `{}` Vote Weight: {:?}",
                    id,
                    index,
                    account_number,
                    account,
                    vote_weight
                );
                if !vote_weight.is_empty() {
                    vote_weight_accounts.push((account, vote_weight));
                }
            }
        }
        Ok(vote_weight_accounts)
    }

    pub async fn account_vote_weight_v1(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Option<Vec<AccountVoteWeightInfoV1>>> {
        let nomination_records = match self.nomination_records(who, hash).await {
            Ok(Some(records)) => Some(
                records
                    .into_iter()
                    .map(|(node, record)| {
                        (
                            node,
                            NominationRecordV1 {
                                nomination: record.nomination,
                                last_vote_weight: record.last_vote_weight.to_string(),
                                last_vote_weight_update: record.last_vote_weight_update,
                                revocations: record.revocations,
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => self.nomination_records_v1(who, hash).await?,
        };
        if let Some(nomination_records) = nomination_records {
            let account_vote_weight = nomination_records
                .into_iter()
                .map(|(account, record)| AccountVoteWeightInfoV1 {
                    node_vote_weight: NodeVoteWeightInfoV1 {
                        account,
                        nomination: record.nomination,
                        weight: calc_account_vote_weight_v1(&record, height),
                    },
                    revocations: record.revocations,
                })
                .collect::<Vec<_>>();
            Ok(Some(account_vote_weight))
        } else {
            Ok(None)
        }
    }

    pub async fn total_node_deposit_weight_v1(
        &self,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Option<TotalDepositWeightInfoV1>> {
        let mut node_deposit_weight = TotalDepositWeightInfoV1::default();
        let psedu_intentions = match self.psedu_intentions(hash).await {
            Ok(Some(psedu_intentions)) => Some(
                psedu_intentions
                    .into_iter()
                    .map(|psedu_intention| PseduIntentionInfoV1 {
                        psedu_intention_common: psedu_intention.psedu_intention_common,
                        psedu_intention_profs: PseduIntentionVoteWeightV1 {
                            last_total_deposit_weight: psedu_intention
                                .psedu_intention_profs
                                .last_total_deposit_weight
                                .to_string(),
                            last_total_deposit_weight_update: psedu_intention
                                .psedu_intention_profs
                                .last_total_deposit_weight_update,
                        },
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => self.psedu_intentions_v1(hash).await?,
        };
        if let Some(psedu_intentions) = psedu_intentions {
            for psedu_intention in psedu_intentions {
                match psedu_intention.psedu_intention_common.id.as_str() {
                    "BTC" => {
                        node_deposit_weight.xbtc = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    "L-BTC" => {
                        node_deposit_weight.lbtc = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    "SDOT" => {
                        node_deposit_weight.sdot = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    _ => unreachable!("Unknown ID"),
                }
            }
            Ok(Some(node_deposit_weight))
        } else {
            Ok(None)
        }
    }

    pub async fn total_node_raw_deposit_weight_v1(
        &self,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Option<TotalDepositWeightInfoV1>> {
        let mut node_deposit_weight = TotalDepositWeightInfoV1::default();
        let psedu_intentions = match self.raw_psedu_intentions(hash).await {
            Ok(Some(psedu_intentions)) => Some(
                psedu_intentions
                    .into_iter()
                    .map(|psedu_intention| PseduIntentionInfoV1 {
                        psedu_intention_common: PseduIntentionInfoCommon {
                            id: psedu_intention.id,
                            circulation: psedu_intention.circulation,
                            price: psedu_intention.price, // will be the default
                            discount: psedu_intention.discount,
                            power: psedu_intention.power, // will be the default
                            jackpot: psedu_intention.jackpot,
                            jackpot_account: psedu_intention.jackpot_account,
                        },
                        psedu_intention_profs: PseduIntentionVoteWeightV1 {
                            last_total_deposit_weight: psedu_intention
                                .last_total_deposit_weight
                                .to_string(),
                            last_total_deposit_weight_update: psedu_intention
                                .last_total_deposit_weight_update,
                        },
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => self.psedu_intentions_v1(hash).await?,
        };
        if let Some(psedu_intentions) = psedu_intentions {
            for psedu_intention in psedu_intentions {
                match psedu_intention.psedu_intention_common.id.as_str() {
                    "BTC" => {
                        node_deposit_weight.xbtc = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    "L-BTC" => {
                        node_deposit_weight.lbtc = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    "SDOT" => {
                        node_deposit_weight.sdot = DepositWeightInfoV1 {
                            balance: psedu_intention.psedu_intention_common.circulation,
                            weight: calc_node_deposit_weight_v1(&psedu_intention, height),
                        };
                    }
                    _ => unreachable!("Unknown ID"),
                }
            }
            Ok(Some(node_deposit_weight))
        } else {
            Ok(None)
        }
    }

    pub async fn total_accounts_deposit_weight_v1(
        &self,
        id: usize,
        accounts: Vec<AccountId>,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Vec<(AccountId, TotalDepositWeightInfoV1)>> {
        let account_number = accounts.len();
        let mut deposit_weight_accounts = vec![];
        for (index, account) in accounts.into_iter().enumerate() {
            let deposit_weight = self
                .account_deposit_weight_v1(&account, hash, height)
                .await?;
            if let Some(deposit_weight) = deposit_weight {
                log::info!(
                    "[{}] ({} / {}) Account `{}` Deposit Weight: {:?}",
                    id,
                    index,
                    account_number,
                    account,
                    deposit_weight
                );
                if !(deposit_weight.xbtc.weight == 0
                    && deposit_weight.lbtc.weight == 0
                    && deposit_weight.sdot.weight == 0)
                {
                    deposit_weight_accounts.push((account, deposit_weight));
                }
            }
        }
        Ok(deposit_weight_accounts)
    }

    async fn account_deposit_weight_v1(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
        height: BlockNumber,
    ) -> Result<Option<TotalDepositWeightInfoV1>> {
        let mut account_deposit_weight = TotalDepositWeightInfoV1::default();
        let psedu_nomination_records = match self.psedu_nomination_records(who, hash).await {
            Ok(Some(records)) => Some(
                records
                    .into_iter()
                    .map(|record| PseduNominationRecordV1 {
                        common: record.common,
                        last_total_deposit_weight: record.last_total_deposit_weight.to_string(),
                        last_total_deposit_weight_update: record.last_total_deposit_weight_update,
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => self.psedu_nomination_records_v1(who, hash).await?,
        };
        if let Some(psedu_nomination_records) = psedu_nomination_records {
            for psedu_nomination_record in psedu_nomination_records {
                match psedu_nomination_record.common.id.as_str() {
                    "BTC" => {
                        account_deposit_weight.xbtc = DepositWeightInfoV1 {
                            balance: psedu_nomination_record.common.balance,
                            weight: calc_account_deposit_weight_v1(
                                &psedu_nomination_record,
                                height,
                            ),
                        };
                    }
                    "L-BTC" => {
                        account_deposit_weight.lbtc = DepositWeightInfoV1 {
                            balance: psedu_nomination_record.common.balance,
                            weight: calc_account_deposit_weight_v1(
                                &psedu_nomination_record,
                                height,
                            ),
                        };
                    }
                    "SDOT" => {
                        account_deposit_weight.sdot = DepositWeightInfoV1 {
                            balance: psedu_nomination_record.common.balance,
                            weight: calc_account_deposit_weight_v1(
                                &psedu_nomination_record,
                                height,
                            ),
                        };
                    }
                    _ => unreachable!("Unknown ID"),
                }
            }
            Ok(Some(account_deposit_weight))
        } else {
            Ok(None)
        }
    }

    pub async fn total_account_assets(
        &self,
        id: usize,
        accounts: Vec<AccountId>,
        hash: Option<Hash>,
    ) -> Result<Vec<(AccountId, Vec<AssetInfo>)>> {
        let account_number = accounts.len();
        let mut account_assets = vec![];
        for (index, account) in accounts.into_iter().enumerate() {
            log::info!(
                "[{}] ({} / {}) Account `{}`",
                id,
                index,
                account_number,
                account
            );
            let assets = self.asset(&account, 0, 10, hash).await?;
            if let Some(assets) = assets {
                let assets = assets.data;
                if !assets.is_empty() {
                    account_assets.push((account, assets));
                }
            }
        }
        Ok(account_assets)
    }
}

/*
fn calc_node_vote_weight(intention: &IntentionInfo, height: BlockNumber) -> u64 {
    let last_total_vote_weight = intention.intention_profs.last_total_vote_weight;
    let duration = height - intention.intention_profs.last_total_vote_weight_update;
    let vote_weight = intention.intention_profs.total_nomination * duration;
    last_total_vote_weight + vote_weight
}

fn calc_account_vote_weight(record: &NominationRecord, height: BlockNumber) -> u64 {
    let last_vote_weight = record.last_vote_weight;
    let duration = height - record.last_vote_weight_update;
    let vote_weight = record.nomination * duration;
    last_vote_weight + vote_weight
}

fn calc_node_deposit_weight(psedu_intention: &PseduIntentionInfo, height: BlockNumber) -> u64 {
    let last_total_deposit_weight = psedu_intention
        .psedu_intention_profs
        .last_total_deposit_weight;
    let duration = height
        - psedu_intention
            .psedu_intention_profs
            .last_total_deposit_weight_update;
    let deposit_weight = psedu_intention.psedu_intention_common.circulation * duration;
    last_total_deposit_weight + deposit_weight
}

fn calc_account_deposit_weight(
    psedu_nomination_record: &PseduNominationRecord,
    height: BlockNumber,
) -> u64 {
    let last_total_deposit_weight = psedu_nomination_record.last_total_deposit_weight;
    let duration = height - psedu_nomination_record.last_total_deposit_weight_update;
    let deposit_weight = psedu_nomination_record.common.balance * duration;
    last_total_deposit_weight + deposit_weight
}
*/

fn calc_node_vote_weight_v1(intention: &IntentionInfoV1, height: BlockNumber) -> u128 {
    let last_total_vote_weight = intention
        .intention_profs
        .last_total_vote_weight
        .parse::<u128>()
        .expect("last_total_vote_weight must be integer; qed");
    let duration = height - intention.intention_profs.last_total_vote_weight_update;
    let vote_weight = intention.intention_profs.total_nomination * duration;
    last_total_vote_weight + u128::from(vote_weight)
}

fn calc_account_vote_weight_v1(record: &NominationRecordV1, height: BlockNumber) -> u128 {
    let last_vote_weight = record
        .last_vote_weight
        .parse::<u128>()
        .expect("last_vote_weight must be integer; qed");
    let duration = height - record.last_vote_weight_update;
    let vote_weight = record.nomination * duration;
    last_vote_weight + u128::from(vote_weight)
}

fn calc_node_deposit_weight_v1(
    psedu_intention: &PseduIntentionInfoV1,
    height: BlockNumber,
) -> u128 {
    let last_total_deposit_weight = psedu_intention
        .psedu_intention_profs
        .last_total_deposit_weight
        .parse::<u128>()
        .expect("last_total_deposit_weight must be integer; qed");
    let duration = height
        - psedu_intention
            .psedu_intention_profs
            .last_total_deposit_weight_update;
    let deposit_weight = psedu_intention.psedu_intention_common.circulation * duration;
    last_total_deposit_weight + u128::from(deposit_weight)
}

fn calc_account_deposit_weight_v1(
    psedu_nomination_record: &PseduNominationRecordV1,
    height: BlockNumber,
) -> u128 {
    let last_total_deposit_weight = psedu_nomination_record
        .last_total_deposit_weight
        .parse::<u128>()
        .expect("last_total_deposit_weight must be integer; qed");
    let duration = height - psedu_nomination_record.last_total_deposit_weight_update;
    let deposit_weight = psedu_nomination_record.common.balance * duration;
    last_total_deposit_weight + u128::from(deposit_weight)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct NodeVoteWeightInfo {
    pub account: AccountId,
    pub nomination: Balance,
    pub weight: u64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct NodeVoteWeightInfoV1 {
    pub account: AccountId,
    pub nomination: Balance,
    pub weight: u128,
}

impl Serialize for NodeVoteWeightInfoV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NodeVoteWeightInfoV1Impl {
            account: self.account,
            nomination: self.nomination,
            weight: self.weight.to_string(),
        }
        .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for NodeVoteWeightInfoV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tmp = NodeVoteWeightInfoV1Impl::deserialize(deserializer)?;
        Ok(NodeVoteWeightInfoV1 {
            account: tmp.account,
            nomination: tmp.nomination,
            weight: tmp
                .weight
                .parse::<u128>()
                .expect("deposit weight must be integer; qed"),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct NodeVoteWeightInfoV1Impl {
    pub account: AccountId,
    pub nomination: Balance,
    pub weight: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AccountVoteWeightInfo {
    #[serde(flatten)]
    pub node_vote_weight: NodeVoteWeightInfo,
    pub revocations: Vec<Revocation>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AccountVoteWeightInfoV1 {
    #[serde(flatten)]
    pub node_vote_weight: NodeVoteWeightInfoV1,
    pub revocations: Vec<Revocation>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TotalDepositWeightInfo {
    pub xbtc: DepositWeightInfo,
    pub lbtc: DepositWeightInfo,
    pub sdot: DepositWeightInfo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DepositWeightInfo {
    pub balance: Balance,
    pub weight: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TotalDepositWeightInfoV1 {
    pub xbtc: DepositWeightInfoV1,
    pub lbtc: DepositWeightInfoV1,
    pub sdot: DepositWeightInfoV1,
}

#[derive(Debug, Default)]
pub struct DepositWeightInfoV1 {
    pub balance: Balance,
    pub weight: u128,
}

impl Serialize for DepositWeightInfoV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DepositWeightInfoV1Impl {
            balance: self.balance,
            weight: self.weight.to_string(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DepositWeightInfoV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tmp = DepositWeightInfoV1Impl::deserialize(deserializer)?;
        Ok(DepositWeightInfoV1 {
            balance: tmp.balance,
            weight: tmp
                .weight
                .parse::<u128>()
                .expect("deposit weight must be integer; qed"),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct DepositWeightInfoV1Impl {
    pub balance: Balance,
    pub weight: String,
}
