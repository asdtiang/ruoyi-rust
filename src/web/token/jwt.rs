use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use crate::error::Error;

/// JWT authentication Token structure
/// diff: 不再保存如此多的信息，都保留在缓存
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JwtClaims {
    //保存在redis的uuid
    pub login_user_key: String,
    //签发时间
    pub exp: usize,
}

impl JwtClaims {
    /// create token_auth
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        match encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JwtClaims encode fail!")), // in practice, you would return the error
        }
    }
    /// verify token_auth invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JwtClaims, Error> {
        let validation = Validation::default();
        match decode::<JwtClaims>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => Err(Error::from("InvalidToken other errors")),
            },
        }
    }
}