use crate::context::CONTEXT;
use crate::error::Error;
use crate::system::domain::vo::SysUserOnlineVO;

use crate::web::token::auth::UserCache;
use crate::web::LOGIN_TOKEN_KEY;
use crate::{error_wrapper_unwrap, RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;

#[pre_authorize("monitor:online:list", user_cache)]
pub async fn list() -> impl IntoResponse {
    error_wrapper_unwrap!(CONTEXT.cache_service.keys(&crate::web::get_login_user_redis_key("*".to_string())),keys);

    let mut user_online_list = vec![];

    for k in keys {
        let c: Result<UserCache, Error> = CONTEXT.cache_service.get_json(&k).await;
        match c {
            Ok(u) => {
                let user_online = SysUserOnlineVO {
                    token_id: Some(u.token_key.trim_start_matches(LOGIN_TOKEN_KEY).to_string()),
                    dept_name: None,
                    user_name: Some(u.user_name),
                    ipaddr: None,
                    login_location: None,
                    phonenumber: None,
                    browser: None,
                    os: None,
                    login_time: Some(u.login_time),
                };
                user_online_list.push(user_online);
            }
            Err(_) => {}
        }
    }
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(user_online_list));
    res.insert("total".to_string(), serde_json::json!(user_online_list.len()));
    res.into_response()
}


#[pre_authorize("system:online:force_logout")]
pub async fn force_logout(token_id: Path<String>) -> impl IntoResponse {
    error_wrapper_unwrap!(CONTEXT.cache_service.del(&crate::web::get_login_user_redis_key(token_id.0)),res);
    if res {
        RespVO::<u64>::from_success_info("强制成功！").into_response()
    } else {
        RespVO::<u64>::from_success_info("强制失败！").into_response()
    }
}
