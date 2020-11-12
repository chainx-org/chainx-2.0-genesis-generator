// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub type AccountId = H256;
pub type AccountIndex = u32;
pub type Address = pallet_indices::address::Address<AccountId, AccountIndex>;

pub type Hash = H256;
pub type BlockNumber = u64;
pub type Balance = u64;
pub type SessionKey = sp_core::ed25519::Public;
pub type Precision = u16;
pub type Name = Vec<u8>;
pub type Token = Vec<u8>;
pub type Memo = Vec<u8>;
pub type URL = Vec<u8>;
pub type Desc = Vec<u8>;
pub type AddrStr = Vec<u8>;
pub type XString = Vec<u8>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageData<T> {
    pub page_total: u32,
    pub page_index: u32,
    pub page_size: u32,
    pub data: Vec<T>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    pub name: String,
    pub details: BTreeMap<AssetType, Balance>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalAssetInfo {
    pub name: String,
    pub token_name: String,
    pub chain: Chain,
    pub precision: u16,
    pub desc: String,
    pub online: bool,
    pub details: BTreeMap<AssetType, Balance>,
    pub limit_props: BTreeMap<AssetLimit, bool>,
}

#[derive(
    PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug, Encode, Decode, Serialize, Deserialize,
)]
pub enum AssetType {
    Free,
    ReservedStaking,
    ReservedStakingRevocation,
    ReservedWithdrawal,
    ReservedDexSpot,
    ReservedDexFuture,
    ReservedCurrency,
    ReservedXRC20,
    GasPayment,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Free
    }
}

#[derive(
    PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug, Encode, Decode, Serialize, Deserialize,
)]
pub enum AssetLimit {
    CanMove,
    CanTransfer,
    CanDeposit,
    CanWithdraw,
    CanDestroyWithdrawal,
    CanDestroyFree,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    token: Token,
    token_name: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentionInfo {
    #[serde(flatten)]
    pub intention_common: IntentionInfoCommon,
    #[serde(flatten)]
    pub intention_profs: IntentionProfs<Balance, BlockNumber>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentionInfoV1 {
    #[serde(flatten)]
    pub intention_common: IntentionInfoCommon,
    #[serde(flatten)]
    pub intention_profs: IntentionProfsV1<Balance, BlockNumber>,
}

impl PartialEq for IntentionInfoV1 {
    fn eq(&self, other: &Self) -> bool {
        self.intention_common.account == other.intention_common.account
    }
}

impl Eq for IntentionInfoV1 {}

impl PartialOrd for IntentionInfoV1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.intention_common
            .account
            .partial_cmp(&other.intention_common.account)
    }
}

impl Ord for IntentionInfoV1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.intention_common
            .account
            .cmp(&other.intention_common.account)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentionInfoCommon {
    /// account id of intention
    pub account: AccountId,
    /// name of intention
    pub name: String,
    /// is validator
    pub is_validator: bool,
    /// how much has intention voted for itself
    pub self_vote: Balance,
    /// jackpot
    pub jackpot: Balance,
    /// jackpot account
    pub jackpot_account: AccountId,

    /// url
    pub url: String,
    /// is running for the validators
    pub is_active: bool,
    /// about
    pub about: String,
    /// session key for block authoring
    pub session_key: AccountId,

    /// is trustee
    pub is_trustee: Vec<Chain>,
}

impl PartialEq for IntentionInfoCommon {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub enum Chain {
    ChainX,
    Bitcoin,
    Ethereum,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::ChainX
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentionProfs<Balance, BlockNumber> {
    pub total_nomination: Balance,
    pub last_total_vote_weight: u64,
    pub last_total_vote_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentionProfsV1<Balance, BlockNumber> {
    pub total_nomination: Balance,
    pub last_total_vote_weight: String,
    pub last_total_vote_weight_update: BlockNumber,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduIntentionInfo {
    #[serde(flatten)]
    pub psedu_intention_common: PseduIntentionInfoCommon,
    #[serde(flatten)]
    pub psedu_intention_profs: PseduIntentionVoteWeight<BlockNumber>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduIntentionInfoV1 {
    #[serde(flatten)]
    pub psedu_intention_common: PseduIntentionInfoCommon,
    #[serde(flatten)]
    pub psedu_intention_profs: PseduIntentionVoteWeightV1<BlockNumber>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduIntentionInfoCommon {
    /// name of intention
    pub id: String,
    /// circulation of id
    pub circulation: Balance,
    pub price: Balance,
    pub discount: u32,
    pub power: Balance,
    /// jackpot
    pub jackpot: Balance,
    /// jackpot account
    pub jackpot_account: AccountId,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize, Decode)]
#[serde(rename_all = "camelCase")]
pub struct PseduIntentionVoteWeight<BlockNumber> {
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduIntentionVoteWeightV1<BlockNumber> {
    pub last_total_deposit_weight: String,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPseduIntentionInfo {
    /// name of intention
    pub id: String,
    /// circulation of id
    pub circulation: Balance,
    pub price: Balance,
    pub discount: u32,
    pub power: Balance,
    /// jackpot
    pub jackpot: Balance,
    /// jackpot account
    pub jackpot_account: AccountId,
    /// vote weight at last update
    pub last_total_deposit_weight: u64,
    /// last update time of vote weight
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NominationRecord {
    pub nomination: Balance,
    pub last_vote_weight: u64,
    pub last_vote_weight_update: BlockNumber,
    pub revocations: Vec<Revocation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NominationRecordV1 {
    pub nomination: Balance,
    pub last_vote_weight: String,
    pub last_vote_weight_update: BlockNumber,
    pub revocations: Vec<Revocation>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revocation {
    pub block_number: BlockNumber,
    pub value: Balance,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduNominationRecord {
    #[serde(flatten)]
    pub common: PseduNominationRecordCommon,
    /// vote weight at last update
    pub last_total_deposit_weight: u64,
    /// last update time of vote weight
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduNominationRecordV1 {
    #[serde(flatten)]
    pub common: PseduNominationRecordCommon,
    /// vote weight at last update
    pub last_total_deposit_weight: String,
    /// last update time of vote weight
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PseduNominationRecordCommon {
    /// name of intention
    pub id: String,
    /// total deposit
    pub balance: Balance,
    pub next_claim: BlockNumber,
}

/// Only used in the early stage that lacking the RPC `chainx_getPseduNominationRecords`
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPseduNominationRecord {
    /// name of intention
    pub id: String,
    /// total deposit
    pub balance: Balance,
    /// vote weight at last update
    pub last_total_deposit_weight: u64,
    /// last update time of vote weight
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct RawDepositVoteWeight<BlockNumber: Default> {
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}
