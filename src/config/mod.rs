use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "canister.toml";

#[derive(Clone, Deserialize, Debug)]
pub struct CanisterConfig {
    pub project: String,
    pub bucket: String,
    pub image: String,
    pub tag: String,
    pub object: String,
    pub path: PathBuf,
    pub proxy: Option<String>,
    pub run_command: RunCommandConfig,
    pub backup_command: BackupCommandConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RunCommandConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BackupCommandConfig {
    pub bucket: String,
    pub path: PathBuf,
}

impl_global_config!(CanisterConfig, GLOBAL_CONFIG);
