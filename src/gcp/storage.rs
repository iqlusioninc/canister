use super::oauth::{self, AuthHeader};
use error::CanisterError;
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
                .proxy(reqwest::Proxy::http(p)?)
                .build(),
            None => reqwest::Client::builder().default_headers(headers).build(),
        }?;
        let gaia_body = storage_client.get(url.as_str()).send()?;

        Ok(gaia_body)
    }
}
