use super::oauth::{self, AuthHeader};
use crate::error::CanisterError;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::Url;
use std::fs::File;

pub struct Storage {
    pub bucket: String,
    pub object: String,
}

impl Storage {
    // https://cloud.google.com/storage/docs/json_api/v1/objects/get
    pub fn get(
        token: &oauth::Token,
        bucket: &str,
        object: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let base = Url::parse("https://www.googleapis.com/storage/v1/b/")?;
        let mut url = base
            .join(&format!("{}/", bucket))?
            .join("o/")?
            .join(&percent_encode(object.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string())?;
        url.set_query(Some("alt=media"));
        dbg!(&url);
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

    // https://cloud.google.com/storage/docs/json_api/v1/objects/list
    pub fn list(
        token: &oauth::Token,
        bucket: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let base = Url::parse("https://www.googleapis.com/storage/v1/b/")?;
        let url = base.join(&format!("{}/", bucket))?.join("o/")?;
        dbg!(&url);
        let headers = token.headers(AuthHeader::Bearer);
        let storage_client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;
        let response = storage_client.get(url.as_str()).send()?;
        dbg!(&response);
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }

    // https://cloud.google.com/storage/docs/json_api/v1/objects/insert
    pub fn insert(
        token: &oauth::Token,
        bucket: &str,
        object: File,
        name: &str,
        proxy: Option<&str>,
    ) -> Result<reqwest::Response, CanisterError> {
        let base = Url::parse("https://www.googleapis.com/upload/storage/v1/b/")?;
        let mut url = base.join(&format!("{}/", bucket))?.join("o")?;
        url.set_query(Some(&format!("uploadType=media&name={}", name)));
        dbg!(&url);

        let mut headers = token.headers(AuthHeader::Bearer);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        dbg!(&headers);

        let storage_client = match proxy {
            Some(p) => reqwest::Client::builder()
                .default_headers(headers)
                .proxy(reqwest::Proxy::all(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;

        let response = storage_client.post(url.as_str()).body(object).send()?;
        dbg!(&response);
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }
}
