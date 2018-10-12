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
}

impl_global_config!(CanisterConfig, GLOBAL_CONFIG);
