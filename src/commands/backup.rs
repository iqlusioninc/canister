use crate::gcp::{Storage, Token};
use crate::packer::Packer;
use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
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
        let bucket = &config.backup.bucket;
        let proxy = config.proxy.as_deref();
        let token = Token::from_gcloud_tool().unwrap_or_else(|e| {
            status_err!("Error, gcloud auth print-access-token cmd failed: {}", e);
            process::exit(1);
        });

        // create tar file
        let path = &config.backup.path;
        let name = &config.backup.name;
        let tar = path.join(name);
        let tar_file = File::create(&tar).unwrap();

        // pack up dir
        let mut packer = Packer::new(tar_file);
        packer.pack().unwrap_or_else(|e| {
            status_err!("Error, uneable to pack archive: {}", e);
            process::exit(1);
        });

        // upload obj to bucket
        let backup = File::open(&tar).unwrap();
        Storage::insert(&token, bucket, backup, name, proxy).unwrap_or_else(|e| {
            status_err!("Error, unable to upload object to bucket: {}", e);
            process::exit(1);
        });
    }
}
