use super::oauth::{self, AuthHeader};
use crate::error::Error;
use crate::https_client::{HttpsClient, Query};
use http::Uri;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

pub const API_HOSTNAME: &str = "www.googleapis.com";

// https://url.spec.whatwg.org/#path-percent-encode-set
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'#')
    .add(b'?')
    .add(b'{')
    .add(b'}');

pub struct Storage {
    https_client: HttpsClient,
    token: oauth::Token,
    bucket: String,
}

impl Storage {
    /// Create a new storage client
    pub fn new(
        bucket: impl Into<String>,
        token: oauth::Token,
        proxy: Option<&Uri>,
    ) -> Result<Self, Error> {
        let https_client = HttpsClient::new(API_HOSTNAME, token, proxy)?;
        let headers = token.headers(AuthHeader::Bearer);

        Ok(Self {
            https_client,
            token,
            bucket: bucket.into(),
        })
    }

    // https://cloud.google.com/storage/docs/json_api/v1/objects/get
    pub fn get(&self, object: &str) -> Result<hyper::Response<hyper::Body>, Error> {
        let path = self.build_request_path(Some(object));
        let mut params = Query::new();
        params.add("alt", "media");

        let response = self
            .https_client
            .get(&path, &params)
            .send()?;
        if !response.status().is_success() {
            panic!("{}", response.status())
        }

        Ok(response)
    }

    // https://cloud.google.com/storage/docs/json_api/v1/objects/list
    pub fn list(&self) -> Result<hyper::Response<hyper::Body>, Error> {
        let path = self.build_request_path(None);

        let response = self.https_client.get(&path, &Query::new()).send()?;
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }

    fn build_request_path(&self, object: Option<&str>) -> String {
        let object =
            percent_encode(object.unwrap_or("").as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string();
        format!("/storage/v1/b/{}/o/{}", &self.bucket, object)
    }
}
