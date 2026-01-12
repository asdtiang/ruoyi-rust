use crate::context::CONTEXT;
use crate::system::domain::dto::{UserAddDTO, UserPageDTO, UserRoleAuthQueryDTO, UserUpdateDTO};
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::vo::SysUserVO;
use crate::utils::password_encoder::PasswordEncoder;
use crate::{error_wrapper_unwrap, export_excel_controller, PageVO, RespJson, RespVO};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

#[pre_authorize("system:user:list", user_cache)]
pub async fn list(dto: Json<UserPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.page(&dto.0, &user_cache).await;
    PageVO::from_result(&vo).into_response()
}

#[pre_authorize("system:user:add", user_cache)]
pub async fn add(arg: crate::ValidatedForm<UserAddDTO>) -> impl IntoResponse {
    let data = arg.0;
    let role_ids = data.role_ids.clone();
    let post_ids = data.post_ids.clone();
    let mut sys_user = SysUser::from(data);
    sys_user.create_by = Some(user_cache.user_name());
    sys_user.create_time = Some(crate::Now!().into());
    let mut password = sys_user.password.clone().unwrap_or_default();

    //初始密码需要更改才能使用
    if password.is_empty() {
        //默认密码
        password = "123456".to_string();
    }

    sys_user.password = Some(PasswordEncoder::encode(&password));

    let rows_affected = CONTEXT
        .sys_user_service
        .add(&sys_user, &role_ids.unwrap_or_default(), &post_ids.unwrap_or_default())
        .await;
    RespVO::from_result(&rows_affected).into_response()
}

//用户编辑，需要查询post和role列表

#[pre_authorize("system:user:query", user_cache)]
pub async fn detail(user_id: Option<Path<String>>) -> impl IntoResponse {
    let mut res = RespJson::success_info("操作成功");

    if user_id.is_some() {
        let user_id = user_id.unwrap().0;

        error_wrapper_unwrap!(CONTEXT.sys_user_service.detail(&user_id), user);

        let user = SysUserVO::from(user);
        let role_ids: Vec<String> = CONTEXT
            .sys_role_service
            .finds_role_ids_by_user_id(&user_id)
            .await
            .unwrap_or_default();
        res.insert("data".to_string(), serde_json::json!(user));
        res.insert("roleIds".to_string(), serde_json::json!(role_ids));

        let post_ids: Vec<String> = CONTEXT
            .sys_post_service
            .finds_post_ids_by_user_id(&user_id)
            .await
            .unwrap_or_default();
        res.insert("postIds".to_string(), serde_json::json!(post_ids));
    }
    res.insert(
        "posts".to_string(),
        serde_json::json!(CONTEXT.sys_post_service.finds_all().await.unwrap_or_default()),
    );
    res.insert(
        "roles".to_string(),
        serde_json::json!(CONTEXT.sys_role_service.finds_all().await.unwrap_or_default()),
    );

    res.into_response()
}

#[pre_authorize("system:user:edit", user_cache)]
pub async fn update(arg: crate::ValidatedForm<UserUpdateDTO>) -> impl IntoResponse {
    let data = arg.0;
    let role_ids = data.role_ids.clone();
    let post_ids = data.post_ids.clone();
    let mut sys_user = SysUser::from(data);
    sys_user.update_by = Some(user_cache.user_name());
    sys_user.update_time = Some(crate::Now!().into());

    let res = CONTEXT
        .sys_user_service
        .update(&sys_user, &role_ids.unwrap_or_default(), &post_ids.unwrap_or_default(),&user_cache)
        .await;
    RespVO::from_result(&res).into_response()
}

#[pre_authorize("system:user:remove", user_cache)]
pub async fn remove(user_id: Path<String>) -> impl IntoResponse {
    let rows_affected = CONTEXT.sys_user_service.remove_batch(&user_id, &user_cache).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功", "删除失败").into_response()
}

#[pre_authorize(user_cache)] //todo 需要加入权限？
pub async fn get_dept_tree() -> impl IntoResponse {
    let dept_tree = CONTEXT.sys_dept_service.get_dept_tree(&user_cache).await;
    RespVO::from_result(&dept_tree).into_response()
}

#[pre_authorize("system:user:query")]
pub async fn set_auth_roles(arg: Query<UserRoleAuthQueryDTO>) -> impl IntoResponse {
    let s = arg.role_ids.clone().unwrap_or_default();
    let role_ids: Vec<String> = s.split(",").map(|s| s.to_string()).collect();
    let _ = CONTEXT
        .sys_user_role_service
        .reset_through_user_id(&arg.user_id.clone().unwrap_or_default(), &role_ids)
        .await;
    RespVO::<u64>::from_success_info("更新成功！").into_response()
}

#[pre_authorize("system:user:query", user_cache)]
pub async fn get_auth_roles(user_id: Path<String>) -> impl IntoResponse {
    let user_id = user_id.0;
    let res = CONTEXT.sys_user_service.get_auth_roles(&user_id, &user_cache).await;
    match res {
        Ok((user, filter_roles)) => {
            let mut res = RespJson::success_info("操作成功");
            res.insert("user".to_string(), serde_json::json!(user));
            res.insert("roles".to_string(), serde_json::json!(filter_roles));
            res.into_response()
        }
        Err(e) => RespVO::<u64>::from_error_info(500, &e.to_string()).into_response(),
    }
}

#[pre_authorize("system:user:resetPwd", user_cache)]
pub async fn reset_pwd(dto: Json<UserUpdateDTO>) -> impl IntoResponse {
    let UserUpdateDTO { user_id, password, .. } = dto.0;
    let res = CONTEXT
        .sys_user_service
        .reset_pwd(&user_id.unwrap_or_default(), &password.unwrap_or_default(), &user_cache)
        .await;
    RespVO::<u64>::judge_result(res, "更新成功！", "更新失败").into_response()
}

//更改用户当前状态

#[pre_authorize("system:user:edit")]
pub async fn change_status(dto: Json<UserUpdateDTO>) -> impl IntoResponse {
    let UserUpdateDTO { user_id, status, .. } = dto.0;
    let res = CONTEXT
        .sys_user_service
        .update_status(&user_id.unwrap_or_default(), status.unwrap_or_default())
        .await;
    RespVO::<u64>::judge_result(res, "更新成功！", "更新失败").into_response()
}
export_excel_controller!(
    "system:user:export",
    UserPageDTO,
    CONTEXT,
    sys_user_service,
    export_as_excel_bytes
);
