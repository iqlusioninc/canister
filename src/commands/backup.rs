use crate::config::CanisterConfig;
use crate::gcp::{Storage, Token};
use abscissa::{Callable, GlobalConfig};
use std::process;

#[derive(Debug, Options)]
pub struct BackupCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for BackupCommand {
    fn default() -> Self {
        Self {
            config: None,
            verbose: false,
        }
    }
}

impl Callable for BackupCommand {
    fn call(&self) {
        let config = CanisterConfig::get_global();
        let bucket = &config.backup_command.bucket;
        let proxy = config.proxy.as_ref().map(String::as_str);
        let token = Token::from_gcloud_tool().unwrap_or_else(|e| {
            status_err!("Error, gcloud auth print-access-token cmd failed: {}", e);
            process::exit(1);
        });

        let response = Storage::list(&token, bucket, proxy).unwrap_or_else(|e| {
            status_err!("Error, unable to list objects from bucket: {}", e);
            process::exit(1);
        });

        debug!("response: {:?}", response)
    }
}
