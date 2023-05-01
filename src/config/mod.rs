use serde::Deserialize;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "canister.toml";

#[derive(Default, Deserialize, Debug)]
pub struct CanisterConfig {
    pub project: String,
    pub bucket: String,
    pub image: String,
    pub tag: String,
    pub object: String,
    pub path: PathBuf,
    pub proxy: Option<String>,
    pub run_command: RunCommandConfig,
}

#[derive(Default, Deserialize, Debug)]
pub struct RunCommandConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

