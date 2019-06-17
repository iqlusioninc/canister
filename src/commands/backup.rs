use crate::gcp::Token;
use crate::packer::Packer;
use crate::prelude::*;
use abscissa::{Command, Runnable};
use std::fs::File;
use std::process;

#[derive(Command, Debug, Options)]
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

impl Runnable for BackupCommand {
    fn run(&self) {
        let config = app_config();
        let _bucket = &config.snapshot.bucket;
        let _proxy = config.proxy.as_ref().map(String::as_str);
        let _token = Token::from_gcloud_tool().unwrap_or_else(|e| {
            status_err!("Error, gcloud auth print-access-token cmd failed: {}", e);
            process::exit(1);
        });

        // create tar file
        let tar_path = &config.snapshot.tar_file;
        let tar_file = File::create(tar_path).unwrap();

        // pack up dir to snapshot
        let mut packer = Packer::new(tar_file);
        packer.pack().unwrap_or_else(|e| {
            status_err!("Error, uneable to pack archive: {}", e);
            process::exit(1);
        });

        // upload snapshot obj to bucket
    }
}
