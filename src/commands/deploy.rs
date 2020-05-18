use crate::gcp::{Manifest, Storage, Token};
use crate::prelude::*;
use crate::unpacker::{HexDigest, Unpacker};
use abscissa_core::{Command, Options, Runnable};
use std::process;

use std::fs;
use std::io;
use std::os::unix;

#[derive(Command, Debug, Options)]
pub struct DeployCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for DeployCommand {
    fn default() -> Self {
        Self {
            config: None,
            verbose: false,
        }
    }
}

impl Runnable for DeployCommand {
    #[allow(clippy::complexity)]
    fn run(&self) {
        let config = app_config();
        let project = &config.project;
        let bucket = &config.bucket;
        let image = &config.image;
        let tag = &config.tag;
        let object_path = &config.object;
        let path = &config.path;
        let proxy = config.proxy.as_deref();
        let token = Token::from_gcloud_tool().unwrap_or_else(|e| {
            status_err!("Error, gcloud auth print-access-token cmd failed: {}", e);
            process::exit(1);
        });

        let (image_id, m) = Manifest::get(&token, project, image, tag, proxy).unwrap_or_else(|e| {
            status_err!("Error, unable to fetch manifest: {}", e);
            process::exit(1);
        });
        debug!("{}", image_id);
        let layers_len = m.layers.len();
        debug!("{:?}", layers_len);
        if layers_len != 1 {
            panic!("layers length more than 1");
        }
        let layer = &m.layers[0];
        debug!("{:?}", layer);
        let layer_digest = HexDigest::new(&layer.digest[7..]);
        debug!("{:?}", &layer_digest);

        let object = format!("{}/sha256:{}", object_path, layer_digest.as_str());
        let response = Storage::get(&token, bucket, &object, proxy).unwrap_or_else(|e| {
            status_err!("Error, unable to download object from bucket: {}", e);
            process::exit(1);
        });
        let mut unpacker = Unpacker::new(response, config.path.join(image_id.to_string()));
        unpacker.unpack().unwrap_or_else(|e| {
            status_err!("Error, unable to unpack archive: {}", e);
            process::exit(1);
        });
        let digest = unpacker.hex_digest();
        debug!("digest: ");
        status_ok!("Downloaded", "{} object from {}", object, bucket);
        debug!("hasher result: {}", digest.as_str());
        debug!("layer digest: {}", layer_digest.as_str());
        assert_eq!(digest, layer_digest);
        let full_path = path.join(image_id.to_string());
        let full_tag = path.join("current");
        if let Err(e) = unix::fs::symlink(&full_path, &full_tag) {
            if e.kind() == io::ErrorKind::AlreadyExists {
                fs::remove_file(&full_tag).unwrap();
                unix::fs::symlink(full_path, full_tag).unwrap();
            }
        }
    }
}
