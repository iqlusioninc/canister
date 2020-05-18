use super::oauth::{self, AuthHeader};
use crate::error::{CanisterError, CanisterErrorKind::*};
use hex;
use reqwest;
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: usize,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub config: Layer,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: usize,
    pub digest: String,
}

pub const SHA256_PREFIX: &str = "sha256:";

pub struct ImageId(String);

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Manifest {
    pub fn get(
        token: &oauth::Token,
        project: &str,
        image: &str,
        tag: &str,
        proxy: Option<&str>,
    ) -> Result<(ImageId, Self), CanisterError> {
        let mut headers = token.headers(AuthHeader::Basic);
        headers.insert(
            ACCEPT,
            "application/vnd.docker.distribution.manifest.v2+json"
                .parse()
                .unwrap(),
        );

        let client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;

        let url = format!("https://gcr.io/v2/{}/{}/manifests/{}", project, image, tag);

        let mut response = client.get(url.as_str()).send()?;

        let docker_digest_header = response
            .headers()
            .get("Docker-Content-Digest")
            .ok_or_else(|| CanisterError::from(err!(ContentDigestMissing, "{}", url)))?
            .to_str()
            .unwrap()
            .to_owned();

        if !docker_digest_header.starts_with(SHA256_PREFIX) {
            panic!("bad digest prefix: {:?}", docker_digest_header);
        }

        let docker_digest = &docker_digest_header[SHA256_PREFIX.len()..];
        debug!("{:?}", docker_digest);
        debug!("response = {:?}", response);

        let body = response.text()?;
        debug!("body = {:?}", body);
        let image_id = ImageId(hex::encode(Sha256::digest(body.as_bytes())));
        assert_eq!(image_id.0, *docker_digest);
        debug!("{}", image_id);

        let manifest = serde_json::from_str(&body)?;

        Ok((image_id, manifest))
    }
}
