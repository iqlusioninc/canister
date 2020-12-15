//! HTTPS Client

use crate::{
    error::{Error, ErrorKind},
    gcp::oauth,
};
use hyper::{
    client::{Client, HttpConnector, ResponseFuture},
    header, Body, Request, Response, Uri,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::iter::FromIterator;

/// User-Agent to send in HTTP request
pub const USER_AGENT: &str = "iqlusion canister";

/// HTTPS Client
pub struct HttpsClient {
    inner: InnerClient,
    hostname: String,
    token: oauth::Token,
}

impl HttpsClient {
    /// Create a new HTTPS client using the provided configuration
    pub fn new(
        hostname: impl Into<String>,
        token: oauth::Token,
        proxy: Option<&Uri>,
    ) -> Result<Self, Error> {
        let inner = match proxy {
            Some(proxy_uri) => {
                let proxy = Proxy::new(Intercept::All, proxy_uri.clone());
                let connector = HttpsConnector::new();
                let proxy_connector = ProxyConnector::from_proxy(connector, proxy)
                    .map_err(|e| ErrorKind::HttpError.context(e))?;
                let client = Client::builder().build(proxy_connector);

                InnerClient::HttpsViaProxy(client)
            }
            None => {
                let client = Client::builder().build(HttpsConnector::new());
                InnerClient::Https(client)
            }
        };

        Ok(Self {
            inner,
            token,
            hostname: hostname.into(),
        })
    }

    /// exposes the ability to sent HTTP GET requests and return responses directly.
    pub async fn get(&self, path: &str, query: &Query) -> Result<Response<Body>, Error> {
        let uri = query.to_request_uri(&self.hostname, path);

        let mut request = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Body::empty())?;

        {
            let headers = request.headers_mut();
            headers.insert(header::AUTHORIZATION, self.token.as_str().parse()?);
            headers.insert(header::CONTENT_TYPE, "application/json".parse()?);
            headers.insert(
                header::USER_AGENT,
                format!("{}/{}", USER_AGENT, env!("CARGO_PKG_VERSION")).parse()?,
            );
        }

        Ok(self.inner.request(request).await?)
    }
}

enum InnerClient {
    Https(Client<HttpsConnector<HttpConnector>>),
    HttpsViaProxy(Client<ProxyConnector<HttpsConnector<HttpConnector>>>),
}

impl InnerClient {
    fn request(&self, req: Request<Body>) -> ResponseFuture {
        match self {
            Self::Https(client) => client.request(req),
            Self::HttpsViaProxy(client) => client.request(req),
        }
    }
}

/// HTTP Query string
/// <https://en.wikipedia.org/wiki/Query_string>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Query(BTreeMap<String, String>);

impl Query {
    /// Create params
    pub fn new() -> Self {
        Self::default()
    }

    /// Add params
    pub fn add(&mut self, field: impl Into<String>, value: impl Into<String>) -> bool {
        //TODO: return result
        self.0.insert(field.into(), value.into()).is_none()
    }

    /// Compute [`Uri`]
    pub fn to_request_uri(&self, hostname: &str, path: &str) -> Uri {
        let path_and_query = format!("{}?{}", path, self);

        Uri::builder()
            .scheme("https")
            .authority(hostname)
            .path_and_query(path_and_query.as_str())
            .build()
            .unwrap()
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (field, value)) in self.0.iter().enumerate() {
            write!(f, "{}={}", field, value)?;

            if i < self.0.len() - 1 {
                write!(f, "&")?;
            }
        }

        Ok(())
    }
}

impl<'a> FromIterator<&'a (String, String)> for Query {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a (String, String)>,
    {
        let mut params = Self::new();

        for (field, value) in iter {
            params.add(field, value);
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::{FromIterator, Query};

    #[test]
    fn params_to_string() {
        let params = Query::from_iter(&[
            ("foo".to_owned(), "value_1".to_owned()),
            ("bar".to_owned(), "value_2".to_owned()),
        ]);

        let serialized_params = params.to_string();
        assert_eq!(&serialized_params, "bar=value_2&foo=value_1");
    }
}
