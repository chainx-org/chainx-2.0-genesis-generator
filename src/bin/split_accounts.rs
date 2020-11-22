use chainx_state_exporter::*;
use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct NewAccount {
    height: u64,
    account: AccountId,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    const SMALL_CHUNK: usize = 10_000;
    const BIG_CHUNK: usize = 100_000;
    for height in (1..=23_000_000).step_by(BIG_CHUNK) {
        let mut accounts = load_accounts::<_, Vec<NewAccount>>(format!(
            "{}-{}.json",
            height,
            height + BIG_CHUNK - 1
        ))?;
        log::info!(
            "load accounts {}-{}.json, account size: {}",
            height,
            height + BIG_CHUNK - 1,
            accounts.len()
        );
        accounts.sort_unstable();

        let mut total_size_counter = 0;
        for h in (height..height + BIG_CHUNK - 1).step_by(SMALL_CHUNK) {
            log::info!("process accounts {}-{}", h, h + SMALL_CHUNK - 1);
            let mut result = Vec::new();
            for account in &accounts {
                if h <= account.height as usize && account.height as usize <= h + SMALL_CHUNK - 1 {
                    result.push(account);
                }
            }
            total_size_counter += result.len();
            log::info!(
                "save accounts {}-{}.json, account size: {}",
                h,
                h + SMALL_CHUNK - 1,
                result.len()
            );
            save_accounts(format!("{}-{}.json", h, h + SMALL_CHUNK - 1), &result)?;
        }
        assert_eq!(total_size_counter, accounts.len());
    }
    Ok(())
}
