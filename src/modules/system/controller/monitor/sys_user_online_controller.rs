use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::context::CONTEXT;
use  crate::system::domain::vo::{SysUserOnlineVO, UserCache};
use crate::error::Error;

use crate::{RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;

//#[get("/online/list")]
#[pre_authorize("monitor:online:list")]
pub async fn list() -> impl IntoResponse {
    let keys = CONTEXT.cache_service.keys(&format!("{}*", LOGIN_TOKEN_KEY)).await.unwrap();

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

//#[delete("/online/{token_id}")]
#[pre_authorize("system:online:force_logout")]
pub async fn force_logout(token_id: Path<String>) -> impl IntoResponse {
    let res = CONTEXT.cache_service.del(&format!("{}{}", LOGIN_TOKEN_KEY, token_id.0)).await.unwrap();
    if res {
        RespVO::<u64>::from_success_info("强制成功！").into_response()
    } else {
        RespVO::<u64>::from_success_info("强制失败！").into_response()
    }
}
