use crate::error::Error;
use crate::RespVO;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    //保存在redis的uuid
    pub login_user_key: String,
    pub user_name: String,
}

impl User {
    pub fn user_name(&self) -> String {

        self.user_name.clone()
    }
    pub fn login_user_key(&self) -> String {

        self.login_user_key.clone()
    }
}

impl From<JwtClaims> for User {
    fn from(claims: JwtClaims) -> Self {
        User {
            login_user_key: claims.login_user_key,
            user_name: claims.user_name,
        }
    }
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync + 'static,
{
    type Rejection = RespVO<String>;

    fn from_request(
        req: Request,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            req.extensions()
                .get::<User>()
                .cloned()
                .ok_or_else(|| RespVO::from_error_info(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized"))
        }
    }
}

/// JWT authentication Token structure
/// diff: 不再保存如此多的信息，都保留在缓存
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JwtClaims {
    //保存在redis的uuid
    pub login_user_key: String,

    //用户名
    pub user_name: String,
    //过期时间
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
