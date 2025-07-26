use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderValue, StatusCode};
use crate::{RespVO, UserCache};
use crate::context::CONTEXT;
use crate::web::token::jwt::JwtClaims;

impl<S> FromRequestParts<S> for UserCache
where
    S: Send + Sync,
{
    type Rejection = RespVO<u64>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(header_value) = parts.headers.get("authorization") {
            let token = get_token(header_value);
            let claims = JwtClaims::verify(&CONTEXT.config.jwt_secret, &token);
            if let Ok(c) = claims {
                let user_cache = CONTEXT.sys_user_service.get_user_cache_by_token(c.login_user_key).await;
                if let Ok(u) = user_cache {
                    let key = u.token_key.clone();
                    let _ = CONTEXT
                        .cache_service
                        .expire(&key, (CONTEXT.config.token_expired_min * 60) as i32)
                        .await;
                    return Ok(u);
                }
            }

            Err(RespVO::from_error_info(u16::from(StatusCode::UNAUTHORIZED), "未授权"))
        } else {
            Err(RespVO::from_error_info(u16::from(StatusCode::UNAUTHORIZED), "未授权"))
        }
    }
}

pub const TOKEN_PREFIX: &'static str = "Bearer ";
fn get_token(header_value: &HeaderValue) -> String {
    match header_value.to_str() {
        Ok(s) => {
            if s.starts_with(TOKEN_PREFIX) {
                s.replace(TOKEN_PREFIX, "")
            } else {
                s.to_string()
            }
        }
        Err(_) => "".to_string(),
    }
}
