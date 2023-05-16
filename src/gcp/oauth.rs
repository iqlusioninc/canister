use crate::error::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::process::Command;
use subtle_encoding::base64;

pub struct Token {
    pub(super) token: String,
}

#[derive(Copy, Clone)]
pub enum AuthHeader {
    Basic,
    Bearer,
}

impl AuthHeader {
    pub fn set(self, headers: &mut HeaderMap, token: &Token) {
        match self {
            AuthHeader::Bearer => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.as_str())).unwrap(),
                );
            }
            AuthHeader::Basic => {
                let password = token.as_str();
                let auth = format!("oauth2accesstoken:{}", password);
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!(
                        "Basic {}",
                        String::from_utf8(base64::encode(auth)).unwrap()
                    ))
                    .unwrap(),
                );
            }
        }
    }
}

impl Token {
    pub fn from_gcloud_tool() -> Result<Token, Error> {
        let cmd = Command::new("gcloud")
            .args(["auth", "print-access-token"])
            .output()
            .expect("gcloud auth print-access-token cmd failed");
        let mut token = String::from_utf8(cmd.stdout)?;
        let len = token.len();
        token.truncate(len - 1);
        Ok(Self { token })
    }

    pub fn as_str(&self) -> &str {
        &self.token
    }

    pub fn headers(&self, header_type: AuthHeader) -> HeaderMap {
        let mut headers = HeaderMap::new();
        header_type.set(&mut headers, self);
        headers
    }
}
