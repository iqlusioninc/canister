use super::oauth::{self, AuthHeader};
use crate::error::CanisterError;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};
use reqwest;

pub struct Storage {
    pub bucket: String,
    pub object: String,
}

impl Storage {
    pub fn get(
        token: &oauth::Token,
        bucket: &str,
        object: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let url = format!(
            "https://www.googleapis.com/storage/v1/b/{}/o/{}?alt=media",
            bucket,
            percent_encode(object.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string()
        );
        debug!("{}", url);
        let headers = token.headers(AuthHeader::Bearer);
        let storage_client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;
        let response = storage_client.get(url.as_str()).send()?;
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }

    // WIP https://cloud.google.com/storage/docs/json_api/v1/objects/list
    pub fn list(
        token: &oauth::Token,
        bucket: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let url = format!("https://www.googleapis.com/storage/v1/b/{}/o", bucket);
        debug!("{}", url);
        let headers = token.headers(AuthHeader::Bearer);
        let storage_client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;
        let response = storage_client.get(url.as_str()).send()?;
        debug!("{:?}", response);
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }

    // WIP https://cloud.google.com/storage/docs/json_api/v1/objects/insert
    pub fn insert(
        token: &oauth::Token,
        bucket: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let url = format!(
            "https://www.googleapis.com/upload/storage/v1/b/{}/o",
            bucket
        );
        debug!("{}", url);
        let headers = token.headers(AuthHeader::Bearer);
        // todo(shella) - add Content-Length and Content-Type to headers
        let storage_client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;

        // POST https://www.googleapis.com/upload/storage/v1/b/bucket/o
        let response = storage_client.post(url.as_str()).send()?;
        //let response = storage_client
        //    .post(url.as_str())
        //    .body(snapshot)
        //    .send()?;
        debug!("{:?}", response);
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }
}
