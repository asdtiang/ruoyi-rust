use crate::context::CONTEXT;
use crate::modules::system::domain::dto::profile::PasswordUpdateDTO;
use crate::system::domain::dto::ProfileUpdateDTO;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::vo::{CommonDeptVO, CommonUserVO};
use crate::{error_wrapper_unwrap, update_marco, RespJson, RespVO};
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
/*
* 用户自身的操作
*/

//的用户信息
#[pre_authorize(user_cache)]
pub async fn profile() -> impl IntoResponse {
    let mut res = RespJson::success_info("操作成功");
    error_wrapper_unwrap!(CONTEXT.sys_user_service.detail(&user_cache.user_id), user);
    let mut user = CommonUserVO::from(user);
    user.dept = Some(CommonDeptVO {
        dept_id: user_cache.dept_id.into(),
        dept_name: user_cache.dept_name.into(),
        leader: None,
    });
    res.insert("data".to_string(), serde_json::json!(user));
    res.insert(
        "postGroup".to_string(),
        serde_json::json!(
            CONTEXT
                .sys_post_service
                .select_post_names_by_user_name(&&user_cache.user_name)
                .await
                .unwrap_or_default()
                .join(",")
        ),
    );
    res.insert(
        "roleGroup".to_string(),
        serde_json::json!(
            user_cache
                .roles
                .clone()
                .into_iter()
                .map(|r| r.role_name.unwrap_or_default())
                .collect::<Vec<_>>()
                .join(",")
        ),
    );
    res.into_response()
}

#[pre_authorize(user_cache)]
pub async fn update_profile(dto: Json<ProfileUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, SysUser);
    data.user_id=Some(user_cache.user_id);
    let res = CONTEXT.sys_user_service.update_profile(data).await;

    RespVO::<u64>::judge_result(res, "修改成功", "修改失败").into_response()
}

//用户自行修改密码
#[pre_authorize(user_cache)]
pub async fn update_pwd(dto: crate::ValidatedForm<PasswordUpdateDTO>) -> impl IntoResponse {
    let PasswordUpdateDTO {
        new_password,
        old_password,
        ..
    } = dto.0;

    let res = CONTEXT
        .sys_user_service
        .change_pwd(
            &new_password.clone().unwrap_or_default(),
            &old_password.clone().unwrap_or_default(),
            &user_cache,
        )
        .await;
    RespVO::<u64>::judge_result(res, "修改密码成功！", "修改密码失败！").into_response()
}
