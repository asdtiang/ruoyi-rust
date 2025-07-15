use crate::context::CONTEXT;
use crate::system::domain::dto::{PasswordUpdateDTO, UserUpdateDTO};
use crate::utils::password_encoder::PasswordEncoder;
use crate::{RespJson, RespVO};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use std::time::Duration;
/*
* 用户自身的操作
*/

//的用户信息
#[pre_authorize(user)]
pub async fn profile() -> impl IntoResponse {
    let user_cache = CONTEXT
        .sys_user_service
        .get_user_cache_by_token(&user.login_user_key)
        .await
        .unwrap();
    let mut res = RespJson::success_info("操作成功");

    res.insert(
        "data".to_string(),
        serde_json::json!(user_cache.user.unwrap()),
    );
      res.insert(
        "postGroup".to_string(),
        serde_json::json!(CONTEXT
            .sys_post_service
            .select_post_names_by_user_name(&&user.user_name)
            .await
            .unwrap_or_default()
            .join(",")),
    );
    res.insert(
        "roleGroup".to_string(),
        serde_json::json!(user_cache
            .roles
            .clone()
            .into_iter()
            .map(|r| r.role_name.unwrap())
            .collect::<Vec<_>>()
            .join(",")),
    );
    res.into_response()
}

//用户自行修改用户信息
#[pre_authorize(user)]
pub async fn update_profile(mut arg: Json<UserUpdateDTO>) -> impl IntoResponse {
    let mut user_cache = CONTEXT
        .sys_user_service
        .get_user_cache_by_token(&user.login_user_key)
        .await
        .unwrap();
    let clone = arg.0.clone();
    arg.0.user_id = user_cache.id.clone().into();
    let res = CONTEXT.sys_user_service.update(arg.0,user.user_name).await.unwrap();
    if res > 0 {
        let mut user = user_cache.user.clone().unwrap();
        user.phonenumber = clone.phonenumber;
        user.email = clone.email;
        user_cache.user = user.into();
        let _ = CONTEXT
            .cache_service
            .set_string_ex(
                &user_cache.token_key,
                &user_cache.to_string(),
                Some(Duration::from_secs(CONTEXT.config.token_expired_min * 60)),
            )
            .await;
    }
    RespVO::from_result(&Ok(res)).into_response()
}

//用户自行修改密码
#[pre_authorize(user)]
pub async fn update_pwd(arg: Query<PasswordUpdateDTO>) -> impl IntoResponse {
    let user_cache = CONTEXT
        .sys_user_service
        .get_user_cache_by_token(&user.login_user_key)
        .await
        .unwrap();
    let user_id = user_cache.id.clone();
    let user = CONTEXT.sys_user_service.find_by_user_id(&user_id).await;
    if user.is_err() {
        return RespVO::from_result(&user).into_response();
    }
    let user = user.unwrap();
    let new_password = &arg.new_password;
    let old_password = &arg.old_password;
    if new_password.eq(old_password) {
        return RespVO::<u64>::from_error_info(500, "新密码不能与旧密码相同").into_response();
    }

    if !PasswordEncoder::verify(
        &user.password.unwrap_or_default(),
        &old_password.clone().unwrap_or_default(),
    ) {
        return RespVO::<u64>::from_error_info(500, "修改密码失败，旧密码错误").into_response();
    }
    let res = CONTEXT
        .sys_user_service
        .update_password_plain(
            &PasswordEncoder::encode(&new_password.clone().unwrap_or_default()),
            &user_id,
        )
        .await;
    RespVO::<u64>::judge_result(res, "修改密码成功！", "修改密码失败！").into_response()
}
