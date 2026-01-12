use crate::utils::web_utils::parse_agent;
use crate::RespVO;
use axum::extract::FromRequestParts;
use axum::http::header::USER_AGENT;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserAgent {
    pub browser: String,
    pub os: String,
}
impl<S> FromRequestParts<S> for UserAgent
where
    S: Send + Sync,
{
    type Rejection = RespVO<u64>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(header_value) = parts.headers.get(USER_AGENT) {
            let user_agent = header_value.to_str().unwrap_or_default();
            let (browser, os) = parse_agent(&user_agent);
            Ok(UserAgent { browser, os })
        } else {
            Ok(UserAgent::default())
        }
    }
}
