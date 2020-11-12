// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

use anyhow::Result;
use chainx_state_exporter::*;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let conf = CmdConfig::init()?;

    let chainx = ChainX::new(&conf.chainx_ws_url).await?;

    let _accounts = chainx
        .new_account(0, vec![2186386, 2186386, 2186386])
        .await?;

    /*
    let storage = chainx.system_events(2186386).await?.unwrap();
    println!("Storage (len = {}): {:?}", storage.0.len(), storage);
    */

    // 36, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 1, 0, 0, 0,
    // system success: 0, 0,
    // 0,
    // 0,
    // 2, 0, 0, 0,
    // assets move: 5, 0, 12, 80, 67, 88, 102, 182, 147, 246, 121, 226, 169, 224, 168, 193, 202, 94, 6, 206, 133, 103, 240, 120, 8, 198, 126, 133, 234, 24, 75, 150, 111, 207, 67, 42, 70, 166, 0, 128, 38, 159, 28, 135, 18, 242, 94, 181, 144, 252, 132, 155, 137, 199, 156, 201, 178, 48, 155, 43, 38, 150, 233, 109, 86, 16, 160, 133, 129, 184, 170, 0, 114, 246, 0, 0, 0, 0, 0, 0, 0, 0,
    // 2, 0, 0, 0,
    // fee producer: 4, 1, 128, 38, 159, 28, 135, 18, 242, 94, 181, 144, 252, 132, 155, 137, 199, 156, 201, 178, 48, 155, 43, 38, 150, 233, 109, 86, 16, 160, 133, 129, 184, 170, 114, 246, 0, 0, 0, 0, 0, 0, 0, 0,
    // 2, 0, 0, 0,
    // assets move: 5, 0, 12, 80, 67, 88, 102, 182, 147, 246, 121, 226, 169, 224, 168, 193, 202, 94, 6, 206, 133, 103, 240, 120, 8, 198, 126, 133, 234, 24, 75, 150, 111, 207, 67, 42, 70, 166, 0, 43, 246, 190, 68, 35, 95, 165, 218, 225, 252, 170, 232, 114, 206, 28, 115, 248, 40, 153, 8, 53, 160, 186, 51, 88, 57, 68, 230, 183, 194, 185, 105, 0, 2, 170, 8, 0, 0, 0, 0, 0
    // 2, 0, 0, 0,
    // fee jackpot: 4, 0, 43, 246, 190, 68, 35, 95, 165, 218, 225, 252, 170, 232, 114, 206, 28, 115, 248, 40, 153, 8, 53, 160, 186, 51, 88, 57, 68, 230, 183, 194, 185, 105, 2, 170, 8, 0, 0, 0, 0, 0,
    // 2, 0, 0, 0
    // bitcoin insert tx: 10, 1, 242, 34, 5, 151, 170, 235, 132, 1, 251, 183, 86, 51, 248, 113, 112, 196, 43, 235, 148, 70, 130, 49, 148, 27, 79, 33, 40, 175, 116, 21, 249, 36, 232, 244, 78, 43, 72, 197, 117, 190, 154, 112, 46, 125, 72, 143, 141, 166, 174, 163, 80, 108, 19, 100, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5
    // 2, 0, 0, 0,
    // assets move: 5, 0, 12, 80, 67, 88, 9, 36, 24, 95, 55, 156, 38, 236, 175, 196, 49, 50, 54, 223, 0, 83, 162, 6, 249, 118, 47, 152, 46, 246, 15, 243, 248, 174, 236, 13, 41, 118, 0, 102, 182, 147, 246, 121, 226, 169, 224, 168, 193, 202, 94, 6, 206, 133, 103, 240, 120, 8, 198, 126, 133, 234, 24, 75, 150, 111, 207, 67, 42, 70, 166, 0, 128, 158, 9, 0, 0, 0, 0, 0
    // 2, 0, 0, 0,
    // system success: 0, 0,
    // 0
    // let events: Vec<EventRecord<ChainXEvent, Hash>> =
    //     codec::Decode::decode(&mut storage.0.as_slice())?;
    // println!("{:?}", events);

    let events = vec![
        EventRecord {
            phase: Phase::ApplyExtrinsic(0),
            event: ChainXEvent::System(SystemEvent::ExtrinsicSuccess),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(1),
            event: ChainXEvent::System(SystemEvent::ExtrinsicSuccess),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XAssets(XAssetsEvent::Move(
                b"PCX".to_vec(),
                Default::default(),
                AssetType::Free,
                Default::default(),
                AssetType::Free,
                255,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XFeeManager(XFeeManagerEvent::FeeForProducer(
                Default::default(),
                255,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XAssets(XAssetsEvent::Move(
                b"PCX".to_vec(),
                Default::default(),
                AssetType::Free,
                Default::default(),
                AssetType::Free,
                255,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XFeeManager(XFeeManagerEvent::FeeForJackpot(
                Default::default(),
                255,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XBitcoin(XBitcoinEvent::InsertTx(
                Default::default(),
                Default::default(),
                TxState::Applying,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::XAssets(XAssetsEvent::Move(
                b"PCX".to_vec(),
                Default::default(),
                AssetType::Free,
                Default::default(),
                AssetType::Free,
                255,
            )),
            topics: Vec::<Hash>::new(),
        },
        EventRecord {
            phase: Phase::ApplyExtrinsic(2),
            event: ChainXEvent::System(SystemEvent::ExtrinsicSuccess),
            topics: Vec::<Hash>::new(),
        },
    ];
    let encode = codec::Encode::encode(&events);
    println!(
        "Storage (len = {}): StorageData({:?})",
        encode.len(),
        encode
    );

    let events: Vec<EventRecord<ChainXEvent, Hash>> =
        codec::Decode::decode(&mut encode.as_slice())?;
    println!("{:?}", events);

    Ok(())
}
