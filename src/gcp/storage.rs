use super::oauth::{self, AuthHeader};
use crate::error::{Error, ErrorKind};
use http::Uri;
use hyper::{
    client::{Client, HttpConnector, ResponseFuture},
    header, Body, Method, Request, Response,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::fs::File;

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
    pub bucket: String,
    pub object: String,
}

impl Storage {
    // https://cloud.google.com/storage/docs/json_api/v1/objects/get
    pub fn get(
        token: &oauth::Token,
        bucket: &str,
        object: &str,
        proxy: Option<&Uri>,
    ) -> Result<hyper::Response<hyper::Body>, Error> {
        let base = "https://www.googleapis.com/storage/v1/b/".parse()?;
        let mut url = base
            .join(&format!("{}/", bucket))?
            .join("o/")?
            .join(&percent_encode(object.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string())?;
        url.set_query(Some("alt=media"));
        let headers = token.headers(AuthHeader::Bearer);
        let storage_client = match proxy {
            Some(proxy_uri) => {
                let proxy = Proxy::new(Intercept::All, proxy_uri.clone());
                proxy.set_header(headers.keys(), headers.values());
                let connector = HttpsConnector::new();
                let proxy_connector = ProxyConnector::from_proxy(connector, proxy)
                    .map_err(|e| ErrorKind::HttpError.context(e))?;
                Client::builder.build(proxy_connector);
            }
            None => Client::builder()
                .default_headers(headers)
                .build(HttpsConnector::new()),
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
        proxy: Option<&Uri>,
    ) -> Result<hyper::Response<hyper::Body>, Error> {
        let base = "https://www.googleapis.com/storage/v1/b/".parse()?;
        let url = base.join(&format!("{}/", bucket))?.join("o/")?;
        let headers = token.headers(AuthHeader::Bearer);
        let storage_client = match proxy {
            Some(proxy_uri) => {
                let proxy = Proxy::new(Intercept::All, proxy_uri.clone());
                proxy.set_headers(headers.keys(), headers.value());
                let connector = HttpsConnector::new();
                let proxy_connector = ProxyConnector::from_proxy(connector, proxy)
                    .map_err(|e| ErrorKind::HttpError.context(e))?;
                Client::builder().build(proxy_connector);
            }
            None => Client::builder().default_headers(headers).build(),
        }?;

        let response = storage_client.get(url.as_str()).send()?;
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }

    // https://cloud.google.com/storage/docs/json_api/v1/objects/insert
    pub async fn insert(
        token: &oauth::Token,
        bucket: &str,
        object: File,
        name: &str,
        proxy: Option<&Uri>,
    ) -> Result<hyper::Response<hyper::Body>, Error> {
        let base :Uri = "https://www.googleapis.com/upload/storage/v1/b/".parse::<Uri>().unwrap();
        let mut url = base.join(&format!("{}/", bucket))?.join("o")?;
        url.set_query(Some(&format!("uploadType=media&name={}", name)));

        let mut headers = token.headers(AuthHeader::Bearer);
        headers.insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/octet-stream"),
        );

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .headers(headers.clone())
            .body(object)?;

        let storage_client = match proxy {
            Some(proxy_uri) => {
                let proxy = Proxy::new(Intercept::All, proxy_uri.clone());
                proxy.set_headers(headers.keys(), headers.value());
                let connector = HttpsConnector::new();
                let proxy_connector = ProxyConnector::from_proxy(connector, proxy)
                    .map_err(|e| ErrorKind::Http.context(e))?;
                Client::builder().build(proxy_connector);
            }
            None => Client::builder().default_headers(headers).build(),
        }?;

        let response = storage_client.request(request).await?;
        //let response = storage_client.post(url.as_str()).body(object).send()?;
        if !response.status().is_success() {
            panic!("{}", response.status())
        }
        Ok(response)
    }
}
