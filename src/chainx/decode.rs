// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

#![allow(non_camel_case_types)]

use std::iter::FromIterator;
use std::mem;

use codec::{Compact, Decode, Encode, Input, Output};
use sp_core::H256;

use light_bitcoin::primitives::H264;

use crate::chainx::types::*;

// fix btree map compact length problem
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct CompatibleBTreeMap<K, V>(pub std::collections::BTreeMap<K, V>);

impl<K: Encode, V: Encode> Encode for CompatibleBTreeMap<K, V> {
    fn size_hint(&self) -> usize {
        mem::size_of::<u32>()
            + mem::size_of::<K>() * self.0.len()
            + mem::size_of::<V>() * self.0.len()
    }

    fn encode_to<W: Output>(&self, dest: &mut W) {
        (self.0.len() as u32).encode_to(dest);

        for i in self.0.iter() {
            i.encode_to(dest);
        }
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for CompatibleBTreeMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> CompatibleBTreeMap<K, V> {
        let mut map = std::collections::BTreeMap::new();
        map.extend(iter);
        CompatibleBTreeMap(map)
    }
}

impl<K: Ord + Decode, V: Decode> Decode for CompatibleBTreeMap<K, V> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        u32::decode(input).and_then(move |len| {
            input.descend_ref()?;
            let result = Result::from_iter((0..len).map(|_| Decode::decode(input)));
            input.ascend_ref();
            result
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub enum Phase {
    /// Applying an extrinsic.
    ApplyExtrinsic(u32),
    /// The end.
    Finalization,
}

/// Record of an event happening.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct EventRecord<E, T> {
    /// The phase of the block it happened in.
    pub phase: Phase,
    /// The event itself.
    pub event: E,
    /// The list of the topics this event has.
    pub topics: Vec<T>,
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub enum ChainXEvent {
    System(SystemEvent),
    Indices(IndicesEvent),
    XSession(XSessionEvent),
    XGrandpa(XGrandpaEvent),
    XFeeManager(XFeeManagerEvent),
    XAssets(XAssetsEvent),
    XRecords(XRecordsEvent),
    XStaking(XStakingEvent),
    XTokens(XTokensEvent),
    XSpot(XSpotEvent),
    XBitcoin(XBitcoinEvent),
    XSdot(XSdotEvent),
    XBridgeFeatures(XBridgeFeaturesEvent),
    XMultisig(XMultisigEvent),
    XFisher(XFisherEvent),
    XBridgeCommon(XBridgeCommonEvent),
    XBitcoinLockup(XBitcoinLockupEvent),
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub enum SystemEvent {
    ExtrinsicSuccess,
    ExtrinsicFailed,
}

pub type AccountIndex = u32;
pub type IndicesEvent = IndicesRawEvent<AccountId, AccountIndex>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum IndicesRawEvent<AccountId, AccountIndex> {
    NewAccountIndex(AccountId, AccountIndex),
}

pub type XSessionEvent = XSessionRawEvent<BlockNumber>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XSessionRawEvent<BlockNumber> {
    NewSession(BlockNumber),
}

pub type XGrandpaEvent = XGrandpaRawEvent<SessionKey>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XGrandpaRawEvent<SessionKey> {
    NewAuthorities(Vec<(SessionKey, u64)>),
}

pub type XFeeManagerEvent = XFeeManagerRawEvent<AccountId, Balance>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XFeeManagerRawEvent<AccountId, Balance> {
    FeeForJackpot(AccountId, Balance),
    FeeForProducer(AccountId, Balance),
    FeeForCouncil(AccountId, Balance),
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub enum SignedBalance {
    Positive(Balance),
    Negative(Balance),
}
pub type XAssetsEvent = XAssetsRawEvent<AccountId, Balance, SignedBalance>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XAssetsRawEvent<AccountId, Balance, SignedBalance> {
    Move(Token, AccountId, AssetType, AccountId, AssetType, Balance),
    Issue(Token, AccountId, Balance),
    Destory(Token, AccountId, Balance),
    Set(Token, AccountId, AssetType, Balance),
    Register(Token, bool),
    Revoke(Token),
    NewAccount(AccountId),
    Change(Token, AccountId, AssetType, SignedBalance),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum ApplicationState {
    Applying,
    Processing,
    NormalFinish,
    RootFinish,
    NormalCancel,
    RootCancel,
}
pub type XRecordsEvent = XRecordsRawEvent<AccountId, Balance>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XRecordsRawEvent<AccountId, Balance> {
    Deposit(AccountId, Token, Balance),
    WithdrawalApply(u32, AccountId, Chain, Token, Balance, Memo, AddrStr),
    WithdrawalFinish(u32, ApplicationState),
}

pub type XStakingEvent = XStakingRawEvent<AccountId, Balance, SessionKey>;
#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XStakingRawEvent<AccountId, Balance, SessionKey> {
    Reward(Balance, Balance),
    MissedBlocksOfOfflineValidatorPerSession(Vec<(AccountId, u32)>),
    EnforceValidatorsInactive(Vec<AccountId>),
    Rotation(Vec<(AccountId, u64)>),
    Unnominate(BlockNumber),
    Nominate(AccountId, AccountId, Balance),
    Claim(u64, u64, Balance),
    Refresh(AccountId, Option<Vec<u8>>, Option<bool>, Option<SessionKey>, Option<Vec<u8>>),
    Unfreeze(AccountId, AccountId),
    SessionReward(Balance, Balance, Balance, Balance),
    ClaimV1(u128, u128, Balance),
    RemoveZombieIntentions(Vec<AccountId>),
}

pub type XTokensEvent = XTokensRawEvent<AccountId, Balance>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XTokensRawEvent<AccountId, Balance> {
    DepositorReward(AccountId, Token, Balance),
    DepositorClaim(AccountId, Token, u64, u64, Balance),
    DepositorClaimV1(AccountId, Token, u128, u128, Balance),
}

pub type Price = u64;
pub type OrderIndex = u64;
pub type TradeHistoryIndex = u64;
pub type TradingPairIndex = u32;
#[derive(PartialEq, Eq, Clone, Debug, Default, Encode, Decode)]
pub struct CurrencyPair(Token, Token);
#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum OrderType {
    Limit,
    Market,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum Side {
    Buy,
    Sell,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum OrderStatus {
    ZeroFill,
    ParitialFill,
    Filled,
    ParitialFillAndCanceled,
    Canceled,
}
pub type XSpotEvent = XSpotRawEvent<AccountId, Balance, BlockNumber, Price>;
#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XSpotRawEvent<AccountId, Balance, BlockNumber, Price> {
    UpdateOrder(AccountId, OrderIndex, Balance, BlockNumber, OrderStatus, Balance, Vec<TradeHistoryIndex>),
    PutOrder(AccountId, OrderIndex, TradingPairIndex, OrderType, Price, Side, Balance, BlockNumber),
    FillOrder(TradeHistoryIndex, TradingPairIndex, Price, AccountId, AccountId, OrderIndex, OrderIndex, Balance, u64),
    UpdateOrderPair(TradingPairIndex, CurrencyPair, u32, u32, bool),
    PriceVolatility(u32),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
pub enum TxState {
    NotApplying,
    Applying,
    Signing,
    Broadcasting,
    Processing,
    Confirming(u32, u32),
    Confirmed,
    Unknown,
}
pub type XBitcoinEvent = XBitcoinRawEvent<AccountId, Balance>;
#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBitcoinRawEvent<AccountId, Balance> {
    InsertHeader(u32, H256, u32, H256, H256, u32, u32, u32, H256),
    InsertTx(H256, H256, TxState),
    Deposit(AccountId, Chain, Token, Balance, Memo, AddrStr, Vec<u8>, TxState),
    DepositPending(AccountId, Chain, Token, Balance, AddrStr),
    Withdrawal(u32, Vec<u8>, TxState),
    CreateWithdrawalProposal(AccountId, Vec<u32>),
    SignWithdrawalProposal(AccountId, bool),
    WithdrawalFatalErr(Vec<u8>, Vec<u8>),
    DropWithdrawalProposal(u32, u32, Vec<u32>),
}

pub type EthereumAddress = [u8; 20];
pub type XSdotEvent = XSdotRawEvent<AccountId, Balance>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XSdotRawEvent<AccountId, Balance> {
    Claimed(AccountId, EthereumAddress, Balance),
}

type BitcoinTrusteeType = light_bitcoin::keys::Public;
type BitcoinAddress = light_bitcoin::keys::Address;
pub type BitcoinTrusteeIntentionProps = TrusteeIntentionProps<BitcoinTrusteeType>;
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct TrusteeIntentionProps<TrusteeEntity> {
    pub about: Vec<u8>,
    pub hot_entity: TrusteeEntity,
    pub cold_entity: TrusteeEntity,
}
pub type BitcoinTrusteeSessionInfo<AccountId> = TrusteeSessionInfo<AccountId, BtcTrusteeAddrInfo>;
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct TrusteeSessionInfo<AccountId, TrusteeAddress> {
    pub trustee_list: Vec<AccountId>,
    pub hot_address: TrusteeAddress,
    pub cold_address: TrusteeAddress,
}
#[derive(PartialEq, Eq, Clone, Debug, Default, Encode, Decode)]
pub struct BtcTrusteeAddrInfo {
    pub addr: BitcoinAddress,
    pub redeem_script: Vec<u8>,
}
pub type XBridgeFeaturesEvent = XBridgeFeaturesRawEvent<AccountId>;
#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeFeaturesRawEvent<AccountId> {
    SetBitcoinTrusteeProps(AccountId, BitcoinTrusteeIntentionProps),
    BitcoinNewTrustees(u32, BitcoinTrusteeSessionInfo<AccountId>),
    BitcoinBinding(AccountId, Option<AccountId>, BitcoinAddress, Option<AccountId>),
    EthereumBinding(AccountId, Option<AccountId>, EthereumAddress, Option<AccountId>),
}

type Proposal = Call;
pub type XMultisigEvent = XMultisigRawEvent<AccountId, Hash, Proposal>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XMultisigRawEvent<AccountId, Hash, Proposal> {
    DeployMultiSig(AccountId, AccountId, u32, u32),
    ExecMultiSig(AccountId, AccountId, Hash, Box<Proposal>),
    Confirm(AccountId, Hash, u32, u64),
    RemoveMultiSigIdFor(AccountId, Hash),
}

pub type XFisherEvent = XFisherRawEvent<AccountId, Balance, BlockNumber>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XFisherRawEvent<AccountId, Balance, BlockNumber> {
    SlashDoubleSigner(BlockNumber, BlockNumber, u64, AccountId, Balance),
}

pub type XBridgeCommonEvent = XBridgeCommonRawEvent<AccountId>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeCommonRawEvent<AccountId> {
    ChannelBinding(Token, AccountId, AccountId),
}

pub type XBitcoinLockupEvent = XBitcoinLockupRawEvent<AccountId>;
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBitcoinLockupRawEvent<AccountId> {
    Lock(AccountId, u64, H256, u32, Vec<u8>),
    Unlock(H256, u32, H256, u32),
    UnlockedFromRoot(H256, u32),
}

// ============================================================================

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum Call {
    Indices(IndicesCall),
    Timestamp(TimestampCall),
    Consensus(ConsensusCall),
    Session(SessionCall),
    FinalityTracker(FinalityTrackerCall),
    Grandpa(GrandpaCall),
    XSystem(XSystemCall),
    XFeeManager(XFeeManagerCall),
    XAssets(XAssetsCall),
    XAssetsRecords(XAssetsRecordsCall),
    XAssetsProcess(XAssetsProcessCall),
    XStaking(XStakingCall),
    XTokens(XTokensCall),
    XSpot(XSpotCall),
    XBridgeOfBTC(XBridgeOfBTCCall),
    XBridgeOfSDOT(XBridgeOfSDOTCall),
    XBridgeFeatures(XBridgeFeaturesCall),
    XMultiSig(XMultiSigCall),
    XFisher(XFisherCall),
    XBridgeOfBTCLockup(XBridgeOfBTCLockupCall),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum IndicesCall {}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum TimestampCall {}

pub type Key = Vec<u8>;
pub type KeyValue = (Vec<u8>, Vec<u8>);
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum ConsensusCall {
    report_misbehavior(Vec<u8>),
    note_offline(/*<T::InherentOfflineReport as InherentOfflineReport>::Inherent*/),
    remark(Vec<u8>),
    set_heap_pages(u64),
    set_code(Vec<u8>),
    set_max_extrinsics_count(u32),
    set_storage(Vec<KeyValue>),
    kill_storage(Vec<Key>),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum SessionCall {}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum FinalityTrackerCall {}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum GrandpaCall {}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XSystemCall {}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XFeeManagerCall {}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct Asset {
    token: Token,
    token_name: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XAssetsCall {
    register_asset(Asset, bool, bool),
    revoke_asset(Token),
    set_balance(Address, Token, CompatibleBTreeMap<AssetType, Balance>),
    transfer(Address, Token, Balance, Memo),
    modify_asset_info(Token, Option<Token>, Option<Desc>),
    set_asset_limit_props(Token, CompatibleBTreeMap<AssetLimit, bool>),
    modify_asset_limit(Token, AssetLimit, bool),
    force_transfer(AccountId, AccountId, Token, Balance, Memo),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XAssetsRecordsCall {
    deposit_from_root(AccountId, Token, Balance),
    withdrawal_from_root(AccountId, Token, Balance),
    fix_withdrawal_state(u32, ApplicationState),
    fix_withdrawal_state_list(Vec<(u32, ApplicationState)>),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XAssetsProcessCall {
    withdraw(Token, Balance, AddrStr, Memo),
    revoke_withdraw(u32),
    modify_token_black_list(Token),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XStakingCall {
    nominate(Address, Balance, Memo),
    renominate(Address, Address, Balance, Memo),
    unnominate(Address, Balance, Memo),
    claim(Address),
    unfreeze(Address, u32),
    refresh(
        Option<URL>,
        Option<bool>,
        Option<SessionKey>,
        Option<XString>,
    ),
    register(Name),
    set_sessions_per_era(#[codec(compact)] BlockNumber),
    set_bonding_duration(#[codec(compact)] BlockNumber),
    set_validator_count(Compact<u32>),
    set_missed_blocks_severity(Compact<u32>),
    set_maximum_intention_count(Compact<u32>),
    set_minimum_penalty(Balance),
    set_distribution_ratio((u32, u32)),
    set_minimum_candidate_threshold((Balance, Balance)),
    set_upper_bond_factor(u32),
    set_nomination_record(
        AccountId,
        AccountId,
        Option<Balance>,
        Option<u64>,
        Option<BlockNumber>,
        Option<(Vec<BlockNumber>, Vec<Balance>)>,
    ),
    set_intention_profs(AccountId, Option<Balance>, Option<u64>, Option<BlockNumber>),
    set_nomination_record_v1(
        AccountId,
        AccountId,
        Option<Balance>,
        Option<u128>,
        Option<BlockNumber>,
        Option<(Vec<BlockNumber>, Vec<Balance>)>,
    ),
    set_intention_profs_v1(
        AccountId,
        Option<Balance>,
        Option<u128>,
        Option<BlockNumber>,
    ),
    remove_zombie_intentions(Vec<AccountId>),
    set_global_distribution_ratio((u32, u32, u32)),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XTokensCall {
    claim(Token),
    set_token_discount(Token, u32),
    set_deposit_reward(Balance),
    set_claim_restriction(Token, (u32, BlockNumber)),
    set_deposit_record(AccountId, Token, Option<u64>, Option<BlockNumber>),
    set_deposit_record_v1(AccountId, Token, Option<u128>, Option<BlockNumber>),
    set_psedu_intention_profs(Token, Option<u64>, Option<BlockNumber>),
    set_psedu_intention_profs_v1(Token, Option<u128>, Option<BlockNumber>),
    set_airdrop_distribution_ratio(Token, u32),
    remove_airdrop_asset(Token),
    set_fixed_cross_chain_asset_power_map(Token, u32),
    remove_cross_chain_asset(Token),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XSpotCall {
    put_order(TradingPairIndex, OrderType, Side, Balance, Price),
    cancel_order(TradingPairIndex, OrderIndex),
    set_cancel_order(AccountId, TradingPairIndex, OrderIndex),
    set_handicap(TradingPairIndex, Price, Price),
    refund_locked(AccountId, Token),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeOfBTCCall {
    push_header(Vec<u8>),
    push_transaction(Vec<u8>),
    create_withdraw_tx(Vec<u32>, Vec<u8>),
    sign_withdraw_tx(Option<Vec<u8>>),
    fix_withdrawal_state_by_trustees(u32, ApplicationState),
    set_btc_withdrawal_fee_by_trustees(Balance),
    remove_tx_and_proposal(Option<H256>, bool),
    set_btc_withdrawal_fee(Balance),
    set_btc_deposit_limit(Balance),
    set_btc_deposit_limit_by_trustees(Balance),
    remove_pending(BitcoinAddress, Option<AccountId>),
    remove_pending_by_trustees(BitcoinAddress, Option<AccountId>),
    set_best_index(H256),
    set_header_confirmed_state(H256, bool),
    handle_transaction(H256),
    set_tx_mark(Vec<(H256, bool)>),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeOfSDOTCall {}

#[derive(Clone, Debug, Eq, PartialEq, Default, Encode, Decode)]
pub struct TrusteeInfoConfig {
    pub min_trustee_count: u32,
    pub max_trustee_count: u32,
}
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeFeaturesCall {
    setup_bitcoin_trustee(XString, H264, H264),
    transition_trustee_session(Chain, Vec<AccountId>),
    transition_trustee_session_by_root(Chain, Vec<AccountId>),
    set_trustee_info_config(Chain, TrusteeInfoConfig),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XMultiSigCall {
    execute(AccountId, Box<Proposal>),
    confirm(AccountId, Hash),
    remove_multi_sig_for(AccountId, Hash),
    transition(Vec<(AccountId, bool)>, u32),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XFisherCall {}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum XBridgeOfBTCLockupCall {
    push_transaction(Vec<u8>),
    release_lock(Vec<(H256, u32)>),
    set_locked_coin_limit((u64, u64)),
    create_lock(Vec<u8>),
}
