// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

mod chainx;
mod cmd;

pub use self::chainx::*;
pub use self::cmd::{CmdConfig, Config};

use std::{env, fs, io::Write};

pub fn log_missing_block_height(height: u64) -> anyhow::Result<()> {
    let mut dir = env::current_dir()?;
    dir.push("accounts");
    fs::create_dir_all(dir.as_path())?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}/missing.log", dir.display()))?;
    file.write(format!("{}\n", height).as_bytes())?;
    Ok(())
}

pub fn accounts_exists<S: AsRef<str>>(filename: S) -> anyhow::Result<bool> {
    let mut path = env::current_dir()?;
    path.push("accounts");
    path.push(filename.as_ref());
    Ok(fs::metadata(path).is_ok())
}

pub fn save_accounts<S, T>(filename: S, value: &T) -> anyhow::Result<()>
where
    S: AsRef<str>,
    T: ?Sized + serde::Serialize,
{
    let mut dir = env::current_dir()?;
    dir.push("accounts");
    fs::create_dir_all(dir.as_path())?;
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/{}", dir.display(), filename.as_ref()))?;
    Ok(serde_json::to_writer_pretty(file, value)?)
}

pub fn load_accounts<S, T>(filename: S) -> anyhow::Result<T>
where
    S: AsRef<str>,
    T: serde::de::DeserializeOwned,
{
    let mut dir = env::current_dir()?;
    dir.push("accounts");
    let file = fs::OpenOptions::new().read(true).open(format!(
        "{}/{}",
        dir.display(),
        filename.as_ref()
    ))?;
    Ok(serde_json::from_reader(file)?)
}

pub fn save_state<S, T>(height: u64, filename: S, value: &T) -> anyhow::Result<()>
where
    S: AsRef<str>,
    T: ?Sized + serde::Serialize,
{
    let mut dir = env::current_dir()?;
    dir.push("state_1.0");
    dir.push(height.to_string());
    fs::create_dir_all(dir.as_path())?;
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/{}", dir.display(), filename.as_ref()))?;
    Ok(serde_json::to_writer_pretty(file, value)?)
}

pub fn load_state<S, T>(height: u64, filename: S) -> anyhow::Result<T>
where
    S: AsRef<str>,
    T: serde::de::DeserializeOwned,
{
    let mut dir = env::current_dir()?;
    dir.push("state_1.0");
    dir.push(height.to_string());
    let file = fs::OpenOptions::new().read(true).open(format!(
        "{}/{}",
        dir.display(),
        filename.as_ref()
    ))?;
    Ok(serde_json::from_reader(file)?)
}
