use abscissa_core::Config;
use serde::Deserialize;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "canister.toml";

#[derive(Config, Default, Deserialize, Debug)]
pub struct CanisterConfig {
    pub project: String,
    pub bucket: String,
    pub image: String,
    pub tag: String,
    pub object: String,
    pub path: PathBuf,
    pub proxy: Option<String>,
    pub run_command: RunCommandConfig,
    pub backup: BackupConfig,
}

#[derive(Config, Default, Deserialize, Debug)]
pub struct RunCommandConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

#[derive(Config, Default, Deserialize, Debug)]
pub struct BackupConfig {
    pub bucket: String,
    pub path: PathBuf,
    pub name: String,
}
