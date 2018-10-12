use error::CanisterError;
use reqwest::header::{Authorization, Basic, Bearer, Headers};
use std::process::Command;

pub struct Token {
    pub(super) token: String,
}

#[derive(Copy, Clone)]
pub enum AuthHeader {
    Basic,
    Bearer,
}

impl AuthHeader {
    pub fn set(self, headers: &mut Headers, token: &Token) {
        match self {
            AuthHeader::Bearer => headers.set(Authorization(Bearer {
                token: token.as_str().to_owned(),
            })),
            AuthHeader::Basic => headers.set(Authorization(Basic {
                username: "oauth2accesstoken".to_owned(),
                password: Some(token.as_str().to_owned()),
            })),
        }
    }
}

impl Token {
    pub fn from_gcloud_tool() -> Result<Token, CanisterError> {
        let cmd = Command::new("gcloud")
            .args(&["auth", "print-access-token"])
            .output()
            .expect("gcloud auth print-access-token cmd failed");
        let mut token = String::from_utf8(cmd.stdout.to_owned())?;
        let len = token.len();
        token.truncate(len - 1);
        Ok(Self { token })
    }

    pub fn as_str(&self) -> &str {
        &self.token
    }

    pub fn headers(&self, header_type: AuthHeader) -> Headers {
        let mut headers = Headers::new();
        header_type.set(&mut headers, self);
        headers
    }
}
