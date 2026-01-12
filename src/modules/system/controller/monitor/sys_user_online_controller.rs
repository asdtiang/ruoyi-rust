use crate::context::CONTEXT;
use crate::error::Error;
use crate::system::domain::vo::SysUserOnlineVO;

use crate::web::token::auth::UserCache;
use crate::web::LOGIN_TOKEN_KEY;
use crate::{error_wrapper_unwrap, RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::utils::address_util::get_real_address_by_ip;

#[pre_authorize("monitor:online:list", user_cache)]
pub async fn list() -> impl IntoResponse {
    error_wrapper_unwrap!(CONTEXT.cache_service.keys(&crate::web::get_login_user_redis_key("*")),keys);

    let mut user_online_list = vec![];

    for k in keys {
        let c: Result<UserCache, Error> = CONTEXT.cache_service.get_json(&k).await;
        match c {
            Ok(u) => {
                let login_location=get_real_address_by_ip(&u.login_ip).await.ok();
                let user_online = SysUserOnlineVO {
                    token_id: Some(u.token_key.trim_start_matches(LOGIN_TOKEN_KEY).to_string()),
                    dept_name: Some(u.dept_name),
                    user_name: Some(u.user_name),
                    ipaddr: Some(u.login_ip),
                    login_location,
                    phonenumber: None,
                    browser: Some(u.browser),
                    os: Some(u.os),
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
pub async fn force_logout(token: Path<String>) -> impl IntoResponse {
 let res=   CONTEXT.sys_user_online_service.force_logout_by_token(&token.0).await;
    if res.is_ok_and(|r|r) {
        RespVO::<u64>::from_success_info("强制成功！").into_response()
    } else {
        RespVO::<u64>::from_success_info("强制失败！").into_response()
    }
}
