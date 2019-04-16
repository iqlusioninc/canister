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
    pub restore_command: RestoreCommandConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BackupCommandConfig {
    pub bucket: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RestoreCommandConfig {
    pub bucket: String,
    pub path: PathBuf,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RunCommandConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

impl_global_config!(CanisterConfig, GLOBAL_CONFIG);
