use crate::gcp::{Storage, Token};
use crate::prelude::*;
use crate::unpacker::Unpacker;
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
        let bucket = &config.backup.bucket;
        let proxy = config.proxy.as_deref();
        let token = Token::from_gcloud_tool().unwrap_or_else(|e| {
            status_err!("Error, gcloud auth print-access-token cmd failed: {}", e);
            process::exit(1);
        });

        let path = &config.backup.path;
        let name = &config.backup.name;

        let response = Storage::get(&token, bucket, name, proxy).unwrap_or_else(|e| {
            status_err!("Error, unable to list objects from bucket: {}", e);
            process::exit(1);
        });

        let mut unpacker = Unpacker::new(response, path);
        unpacker.unpack().unwrap_or_else(|e| {
            status_err!("Error, unable to unpack archive: {}", e);
            process::exit(1);
        });
    }
}
