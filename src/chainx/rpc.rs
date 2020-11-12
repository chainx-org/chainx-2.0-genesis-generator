// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use anyhow::Result;
use codec::{Decode, Encode};
use serde_json::{from_value as from_json_value, to_value as to_json_value};
use sp_core::{
    blake2_256,
    crypto::UncheckedFrom,
    ed25519,
    storage::{StorageData, StorageKey},
    twox_128, Hasher,
};
use sp_runtime::traits::BlakeTwo256;
use web3::{BatchTransport, Transport};

use crate::chainx::{decode::CompatibleBTreeMap, types::*, ChainX};

impl ChainX {
    /// Get a block hash, returns hash of latest block by default
    pub async fn block_hash(&self, block_number: Option<BlockNumber>) -> Result<Option<Hash>> {
        let params = vec![to_json_value(block_number)?];
        let hash = self.client.execute("chain_getBlockHash", params).await?;
        let hash = from_json_value(hash)?;
        Ok(hash)
    }

    /// Fetch a storage data by key
    pub async fn storage(
        &self,
        key: &StorageKey,
        hash: Option<Hash>,
    ) -> Result<Option<StorageData>> {
        let params = vec![to_json_value(key)?, to_json_value(hash)?];
        let data = self.client.execute("state_getStorage", params).await?;
        let data = from_json_value(data)?;
        log::debug!("state_getStorage {:?}", data);
        Ok(data)
    }

    /// 获取用户资产信息
    pub async fn asset(
        &self,
        who: &AccountId,
        index: u32,
        size: u32,
        hash: Option<Hash>,
    ) -> Result<Option<PageData<AssetInfo>>> {
        let params = vec![
            to_json_value(who)?,
            to_json_value(index)?,
            to_json_value(size)?,
            to_json_value(hash)?,
        ];
        let data = self
            .client
            .execute("chainx_getAssetsByAccount", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getAssetsByAccount {:?}", data);
        Ok(data)
    }

    /// 批量获取用户资产信息
    pub async fn asset_batch(
        &self,
        accounts: &[AccountId],
        index: u32,
        size: u32,
        hash: Option<Hash>,
    ) -> Result<Vec<Option<PageData<AssetInfo>>>> {
        let mut requests = Vec::with_capacity(accounts.len());
        for account in accounts {
            let params = vec![
                to_json_value(account)?,
                to_json_value(index)?,
                to_json_value(size)?,
                to_json_value(hash)?,
            ];
            let request = self.client.prepare("chainx_getAssetsByAccount", params);
            requests.push(request);
        }
        let responses = self.client.send_batch(requests).await?;

        let mut data = Vec::with_capacity(responses.len());
        for response in responses {
            let response = from_json_value(response?)?;
            log::debug!("chainx_getAssetsByAccount {:?}", response);
            data.push(response);
        }
        Ok(data)
    }

    /// 获取资产信息
    pub async fn assets(
        &self,
        index: u32,
        size: u32,
        hash: Option<Hash>,
    ) -> Result<Option<PageData<TotalAssetInfo>>> {
        let params = vec![
            to_json_value(index)?,
            to_json_value(size)?,
            to_json_value(hash)?,
        ];
        let data = self.client.execute("chainx_getAssets", params).await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getAssets {:?}", data);
        Ok(data)
    }

    /// 获取单个节点信息
    pub async fn intention(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<IntentionInfo>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getIntentionByAccount", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getIntentionByAccount {:?}", data);
        Ok(data)
    }

    /// 获取单个节点信息
    pub async fn intention_v1(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<IntentionInfoV1>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getIntentionByAccountV1", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getIntentionByAccountV1 {:?}", data);
        Ok(data)
    }

    /// 获取节点列表
    pub async fn intentions(&self, hash: Option<Hash>) -> Result<Option<Vec<IntentionInfo>>> {
        let params = vec![to_json_value(hash)?];
        let data = self.client.execute("chainx_getIntentions", params).await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getIntentions {:?}", data);
        Ok(data)
    }

    /// 获取节点列表
    pub async fn intentions_v1(&self, hash: Option<Hash>) -> Result<Option<Vec<IntentionInfoV1>>> {
        let params = vec![to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getIntentionsV1", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getIntentionsV1 {:?}", data);
        Ok(data)
    }

    /// 用户投票信息
    pub async fn nomination_records(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<(AccountId, NominationRecord)>>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getNominationRecords", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getNominationRecords {:?}", data);
        Ok(data)
    }

    /// 用户投票信息
    pub async fn nomination_records_v1(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<(AccountId, NominationRecordV1)>>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getNominationRecordsV1", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getNominationRecordsV1 {:?}", data);
        Ok(data)
    }

    /// 批量用户投票信息
    pub async fn nomination_records_v1_batch(
        &self,
        accounts: &[AccountId],
        hash: Option<Hash>,
    ) -> Result<Vec<Option<Vec<(AccountId, NominationRecordV1)>>>> {
        let mut requests = Vec::with_capacity(accounts.len());
        for account in accounts {
            let params = vec![to_json_value(account)?, to_json_value(hash)?];
            let request = self.client.prepare("chainx_getNominationRecordsV1", params);
            requests.push(request);
        }
        let responses = self.client.send_batch(requests).await?;

        let mut data = Vec::with_capacity(responses.len());
        for response in responses {
            let response = from_json_value(response?)?;
            log::debug!("chainx_getNominationRecordsV1 {:?}", response);
            data.push(response);
        }
        Ok(data)
    }

    /// 充值挖矿列表
    pub async fn psedu_intentions(
        &self,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<PseduIntentionInfo>>> {
        let params = vec![to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getPseduIntentions", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getPseduIntentions {:?}", data);
        Ok(data)
    }

    /// 充值挖矿列表
    pub async fn psedu_intentions_v1(
        &self,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<PseduIntentionInfoV1>>> {
        let params = vec![to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getPseduIntentionsV1", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getPseduIntentionsV1 {:?}", data);
        Ok(data)
    }

    /// 充值挖矿列表
    pub async fn raw_psedu_intentions(
        &self,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<RawPseduIntentionInfo>>> {
        let mut psedu_intentions = Vec::new();

        let hashed_key = twox_128(b"XTokens PseduIntentions").to_vec();
        if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
            log::debug!("raw_psedu_intentions - XTokens PseduIntentions {:?}", data);
            let tokens: Vec<Token> = Decode::decode(&mut data.0.as_slice())?;

            let mut jackpot_account_list = vec![];
            for token in tokens.clone() {
                let mut unhashed_key = b"XAssets AssetInfo".to_vec();
                Encode::encode_to(&token.clone(), &mut unhashed_key);
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    let asset_info: (Asset, bool, BlockNumber) =
                        Decode::decode(&mut data.0.as_slice())?;
                    let init_number = asset_info.2;
                    let mut bytes = Vec::new();
                    bytes.extend_from_slice(BlakeTwo256::hash(&token).as_bytes());
                    bytes.extend_from_slice(BlakeTwo256::hash(&init_number.encode()).as_bytes());
                    let jackpot_account =
                        ed25519::Public::unchecked_from(BlakeTwo256::hash(&bytes[..]));
                    jackpot_account_list.push(jackpot_account);
                }
            }

            for (token, jackpot_account) in tokens.into_iter().zip(jackpot_account_list) {
                let mut info = RawPseduIntentionInfo::default();

                let mut unhashed_key = b"XTokens PseduIntentionProfiles".to_vec();
                Encode::encode_to(&token.clone(), &mut unhashed_key);
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    let vote_weight: PseduIntentionVoteWeight<Balance> =
                        Decode::decode(&mut data.0.as_slice())?;

                    let mut unhashed_key = b"XAssets AssetBalance".to_vec();
                    Encode::encode_to(
                        &(jackpot_account.clone(), b"PCX".to_vec()),
                        &mut unhashed_key,
                    );
                    let hashed_key = blake2_256(&unhashed_key).to_vec();
                    let map = match self.storage(&StorageKey(hashed_key), hash).await? {
                        Some(data) => {
                            let map: CompatibleBTreeMap<AssetType, Balance> =
                                Decode::decode(&mut data.0.as_slice())?;
                            map
                        }
                        None => CompatibleBTreeMap(Default::default()),
                    };
                    let free = map
                        .0
                        .get(&AssetType::Free)
                        .map(|free| *free)
                        .unwrap_or_default();
                    info.jackpot = free;
                    info.jackpot_account = jackpot_account.into();
                    info.last_total_deposit_weight = vote_weight.last_total_deposit_weight;
                    info.last_total_deposit_weight_update =
                        vote_weight.last_total_deposit_weight_update;
                }

                let mut unhashed_key = b"XTokens TokenDiscount".to_vec();
                Encode::encode_to(&token, &mut unhashed_key);
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    let discount: u32 = Decode::decode(&mut data.0.as_slice())?;
                    info.discount = discount;
                }

                // ignore the `price` and `power`, so `price` and `power` will be the default value

                let mut unhashed_key = b"XAssets TotalAssetBalance".to_vec();
                Encode::encode_to(&token, &mut unhashed_key);
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    let total_asset_balance: CompatibleBTreeMap<AssetType, Balance> =
                        Decode::decode(&mut data.0.as_slice())?;
                    info.circulation = total_asset_balance.0.iter().fold(0, |acc, (_, v)| acc + *v);
                }

                info.id = String::from_utf8_lossy(&token).into_owned();
                psedu_intentions.push(info);
            }
        }

        log::debug!("raw_psedu_intentions {:?}", psedu_intentions);
        Ok(Some(psedu_intentions))
    }

    /// 用户充值信息
    pub async fn psedu_nomination_records(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<PseduNominationRecord>>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getPseduNominationRecords", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getPseduNominationRecords {:?}", data);
        Ok(data)
    }

    /// 用户充值信息
    pub async fn psedu_nomination_records_v1(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<PseduNominationRecordV1>>> {
        let params = vec![to_json_value(who)?, to_json_value(hash)?];
        let data = self
            .client
            .execute("chainx_getPseduNominationRecordsV1", params)
            .await?;
        let data = from_json_value(data)?;
        log::debug!("chainx_getPseduNominationRecordsV1 {:?}", data);
        Ok(data)
    }

    /// 批量用户充值信息
    pub async fn psedu_nomination_records_v1_batch(
        &self,
        accounts: &[AccountId],
        hash: Option<Hash>,
    ) -> Result<Vec<Option<Vec<PseduNominationRecordV1>>>> {
        let mut requests = Vec::with_capacity(accounts.len());
        for account in accounts {
            let params = vec![to_json_value(account)?, to_json_value(hash)?];
            let request = self
                .client
                .prepare("chainx_getPseduNominationRecordsV1", params);
            requests.push(request);
        }
        let responses = self.client.send_batch(requests).await?;

        let mut data = Vec::with_capacity(responses.len());
        for response in responses {
            let response = from_json_value(response?)?;
            log::debug!("chainx_getPseduNominationRecordsV1 {:?}", response);
            data.push(response);
        }
        Ok(data)
    }

    /// 用户充值信息
    pub async fn raw_psedu_nomination_records(
        &self,
        who: &AccountId,
        hash: Option<Hash>,
    ) -> Result<Option<Vec<RawPseduNominationRecord>>> {
        let mut psedu_records = Vec::new();

        let hashed_key = twox_128(b"XTokens PseduIntentions").to_vec();
        if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
            log::debug!(
                "raw_psedu_nomination_records - XTokens PseduIntentions {:?}",
                data
            );
            let tokens: Vec<Token> = Decode::decode(&mut data.0.as_slice())?;
            for token in tokens {
                let mut record = RawPseduNominationRecord::default();

                let mut unhashed_key = b"XTokens DepositRecords".to_vec();
                Encode::encode_to(
                    &(ed25519::Public::unchecked_from(who.clone()), token.clone()),
                    &mut unhashed_key,
                );
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    log::debug!(
                        "raw_psedu_nomination_records - XTokens DepositRecords {:?}",
                        data
                    );
                    let vote_weight: RawDepositVoteWeight<BlockNumber> =
                        Decode::decode(&mut data.0.as_slice())?;
                    record.last_total_deposit_weight = vote_weight.last_deposit_weight;
                    record.last_total_deposit_weight_update =
                        vote_weight.last_deposit_weight_update;
                }

                let mut unhashed_key = b"XAssets AssetBalance".to_vec();
                Encode::encode_to(
                    &(ed25519::Public::unchecked_from(who.clone()), token.clone()),
                    &mut unhashed_key,
                );
                let hashed_key = blake2_256(&unhashed_key).to_vec();
                if let Some(data) = self.storage(&StorageKey(hashed_key), hash).await? {
                    log::debug!(
                        "raw_psedu_nomination_records - XAssets AssetBalance {:?}",
                        data
                    );
                    let balances: CompatibleBTreeMap<AssetType, Balance> =
                        Decode::decode(&mut data.0.as_slice())?;
                    record.balance = balances.0.iter().fold(0, |acc, (_, v)| acc + *v);
                }

                record.id = String::from_utf8_lossy(&token).into_owned();

                psedu_records.push(record);
            }
        }
        log::debug!("raw_psedu_nomination_records {:?}", psedu_records);
        Ok(Some(psedu_records))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string_pretty;
    use url::Url;

    fn hex2account(hex: &str) -> Result<AccountId> {
        if hex.starts_with("0x") {
            assert_eq!(hex.len(), 64 + 2);
            let bytes = hex::decode(&hex.as_bytes()[2..])?;
            Ok(AccountId::from_slice(&bytes))
        } else {
            assert_eq!(hex.len(), 64);
            let bytes = hex::decode(hex)?;
            Ok(AccountId::from_slice(&bytes))
        }
    }

    fn laocius() -> AccountId {
        hex2account("0xfa6efb5db13089b4712305e39d0a16867c6822e3b1f4c4619937ae8a21961030").unwrap()
    }

    const HEIGHT: BlockNumber = 4_670_000;

    async fn new_chainx() -> Result<ChainX> {
        env_logger::init();
        const CHAINX_WS_URL: &str = "wss://w1.chainx.org/ws";
        let url = CHAINX_WS_URL.parse::<Url>()?;
        Ok(ChainX::new(&url).await?)
    }

    #[async_std::test]
    async fn test_block_hash() -> Result<()> {
        let chainx = new_chainx().await?;
        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        println!("Hash: {:?}", hash);
        Ok(())
    }

    #[async_std::test]
    async fn test_asset() -> Result<()> {
        let chainx = new_chainx().await?;

        let laocius = laocius();
        let hash = chainx.block_hash(Some(HEIGHT)).await?;

        let asset = chainx.asset(&laocius, 0, 10, hash).await?.unwrap();
        println!("Asset: {}", to_string_pretty(&asset)?);
        Ok(())
    }

    #[async_std::test]
    async fn test_assets() -> Result<()> {
        let chainx = new_chainx().await?;

        let hash = chainx.block_hash(Some(HEIGHT)).await?;

        let assets = chainx.assets(0, 10, hash).await?.unwrap();
        println!("Assets: {}", to_string_pretty(&assets)?);
        Ok(())
    }

    #[async_std::test]
    async fn test_intention() -> Result<()> {
        let chainx = new_chainx().await?;

        let laocius = laocius();

        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        let intention = chainx.intention(&laocius, hash).await?.unwrap();
        println!("Intention: {}", to_string_pretty(&intention)?);

        let intention_v1 = chainx.intention_v1(&laocius, None).await?.unwrap();
        println!("IntentionV1: {}", to_string_pretty(&intention_v1)?);

        Ok(())
    }

    #[async_std::test]
    async fn test_intentions() -> Result<()> {
        let chainx = new_chainx().await?;

        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        let intentions = chainx.intentions(hash).await?.unwrap();
        println!(
            "Intentions (Size = {}): {}",
            intentions.len(),
            to_string_pretty(&intentions)?
        );

        let intentions_v1 = chainx.intentions_v1(None).await?.unwrap();
        println!(
            "IntentionsV1 (Size = {}): {}",
            intentions_v1.len(),
            to_string_pretty(&intentions_v1)?
        );

        Ok(())
    }

    #[async_std::test]
    async fn test_nomination_records() -> Result<()> {
        let chainx = new_chainx().await?;

        let laocius = laocius();

        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        let nomination_records = chainx.nomination_records(&laocius, hash).await?.unwrap();
        println!(
            "NominationRecords: {}",
            to_string_pretty(&nomination_records)?
        );

        let nomination_records_v1 = chainx.nomination_records_v1(&laocius, None).await?.unwrap();
        println!(
            "NominationRecordsV1: {}",
            to_string_pretty(&nomination_records_v1)?
        );

        Ok(())
    }

    #[async_std::test]
    async fn test_psedu_intentions() -> Result<()> {
        let chainx = new_chainx().await?;

        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        let psedu_intentions = chainx.psedu_intentions(hash).await?.unwrap();
        println!("PseduIntentions: {}", to_string_pretty(&psedu_intentions)?);

        let raw_psedu_intentions = chainx.raw_psedu_intentions(hash).await?.unwrap();
        println!(
            "RawPseduIntentions: {}",
            to_string_pretty(&raw_psedu_intentions)?
        );

        let psedu_intentions_v1 = chainx.psedu_intentions_v1(None).await?.unwrap();
        println!(
            "PseduIntentionsV1: {}",
            to_string_pretty(&psedu_intentions_v1)?
        );

        Ok(())
    }

    #[async_std::test]
    async fn test_psedu_nomination_records() -> Result<()> {
        let chainx = new_chainx().await?;

        let laocius = laocius();

        let hash = chainx.block_hash(Some(HEIGHT)).await?;
        let psedu_nomination_records = chainx
            .psedu_nomination_records(&laocius, hash)
            .await?
            .unwrap();
        println!(
            "PseduNominationRecords: {}",
            to_string_pretty(&psedu_nomination_records)?
        );

        let raw_psedu_nomination_records = chainx
            .raw_psedu_nomination_records(&laocius, hash)
            .await?
            .unwrap();
        println!(
            "Raw PseduNominationRecords: {}",
            to_string_pretty(&raw_psedu_nomination_records)?
        );

        let psedu_nomination_records_v1 = chainx
            .psedu_nomination_records_v1(&laocius, None)
            .await?
            .unwrap();
        println!(
            "PseduNominationRecordsV1: {}",
            to_string_pretty(&psedu_nomination_records_v1)?
        );

        Ok(())
    }
}
