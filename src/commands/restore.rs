use crate::gcp::{Storage, Token};
use crate::prelude::*;
use abscissa_core::{Command, Runnable};
use std::process;

#[derive(Command, Debug, Options)]
pub struct RestoreCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for RestoreCommand {
    fn default() -> Self {
        Self {
            config: None,
            verbose: false,
        }
    }
}

impl Runnable for RestoreCommand {
    fn run(&self) {
        let config = app_config();
        let bucket = &config.snapshot.bucket;
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
