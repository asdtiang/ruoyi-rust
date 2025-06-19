pub mod log;

use crate::config::global_constants::LOGIN_TOKEN_KEY;
use crate::config::global_constants::ADMIN_NAME;
use crate::context::CONTEXT;
use crate::error::Error;
use crate::system::domain::vo::{JWTToken, UserCache};
use crate::RespVO;
use axum::http::HeaderMap;

pub const TOKEN_PREFIX: &'static str = "Bearer ";

pub fn get_token(header_map: &HeaderMap) -> String {
    let mut token = header_map
        .get("authorization")
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_default();
    if token.starts_with(TOKEN_PREFIX) {
        token = token.replace(TOKEN_PREFIX, "");
    }
    token
}

///Check whether the token_auth is valid and has not expired
pub async fn checked_token(token: &str) -> Result<UserCache, Error> {
    //check token_auth alive
    let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match claims {
        Ok(c) => {
            let key = format!("{}{}", LOGIN_TOKEN_KEY, c.login_user_key);
            let user_cache: Result<UserCache, Error> = CONTEXT.cache_service.get_json(&key).await;
            match user_cache {
                Ok(u) => {
                    //刷新过期时间
                    CONTEXT
                        .cache_service
                        .expire(&key, (CONTEXT.config.token_expired_min * 60) as i32)
                        .await
                        .expect("TODO: panic message");
                    Ok(u)
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(crate::error::Error::from(e.to_string())),
    }
}

///Permission to check
/// permit_str支持与非 如sys:user:list||sys:user:delete，暂时不实现，只支持一个权限
pub async fn check_auth(user_cache: &UserCache, permit_str: &str) -> Result<(), Error> {
    let permit_str = permit_str.replace("\"", "");
    if permit_str.len() == 0 {
        return Ok(());
    }
    if user_cache.user_name == ADMIN_NAME {
        return Ok(());
    }

    //let sys_menu = CONTEXT.sys_menu_service.all().await?;
    //权限校验
    for cache_permission in &user_cache.permissions {
        if cache_permission.eq(&permit_str) {
            return Ok(());
        }
    }
    Err(crate::error::Error::from(format!(
        "无权限访问{}",
        permit_str
    )))
}

//鉴权
pub async fn check_permit(header_map: HeaderMap, permit_str: &str) -> Option<RespVO<u64>> {
    let token = get_token(&header_map);
    match checked_token(&token).await {
        Ok(data) => {
            match check_auth(&data, permit_str).await {
                Ok(_) => {
                    crate::web_data::set_token(token);
                    crate::web_data::set_user_name(data.user_name);
                }
                Err(e) => {
                    //仅提示拦截
                    let resp: RespVO<u64> = RespVO {
                        code: 500,
                        msg: Some(e.to_string()),
                        data: None,
                    };
                    return Some(resp);
                }
            }
        }
        Err(e) => {
            //401 http状态码会强制前端退出当前登陆状态
            let resp: RespVO<u64> = RespVO {
                code: 401,
                msg: Some(format!("Unauthorized for:{}", e.to_string())),
                data: None,
            };
            return Some(resp);
        }
    }
    None
}
