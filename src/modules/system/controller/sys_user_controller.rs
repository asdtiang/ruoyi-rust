use crate::config::global_constants::ADMIN_NAME;
use crate::context::CONTEXT;
use crate::system::domain::dto::{UserAddDTO, UserPageDTO, UserRoleAuthQueryDTO, UserUpdateDTO};
use crate::system::domain::vo::SysUserVO;
use crate::web_data::get_user_name;
use crate::{export_excel_controller, PageVO, RespJson, RespVO};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

//#[get("/user/list")]
#[pre_authorize("system:user:list")]
pub async fn list(dto: Json<UserPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.page(&dto.0).await;
    PageVO::from_result(&vo).into_response()
}

//#[post("/user")]
#[pre_authorize("system:user:add")]
pub async fn add(arg: axum_valid::Valid<Json<UserAddDTO>>) -> impl IntoResponse {
    let rows_affected = CONTEXT.sys_user_service.add(&arg.0 .0).await;

    RespVO::from_result(&rows_affected).into_response()
}

//用户编辑，需要查询post和role列表
//#[get("/user/{user_id}")]
#[pre_authorize("system:user:query")]
pub async fn detail(user_id: Option<Path<String>>) -> impl IntoResponse {
    let mut res = RespJson::success_info("操作成功");

    if user_id.is_some() {
        let user_id = user_id.unwrap().0;
        let user = CONTEXT.sys_user_service.detail(&user_id).await;
        if user.is_err() {
            return RespVO::from_result(&user).into_response();
        }
        let user = SysUserVO::from(user.unwrap());
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
        serde_json::json!(CONTEXT
            .sys_post_service
            .finds_all()
            .await
            .unwrap_or_default()),
    );
    res.insert(
        "roles".to_string(),
        serde_json::json!(CONTEXT
            .sys_role_service
            .finds_all()
            .await
            .unwrap_or_default()),
    );

    res.into_response()
}

//#[put("/user")]
#[pre_authorize("system:user:edit")]
pub async fn update(arg: axum_valid::Valid<Json<UserUpdateDTO>>) -> impl IntoResponse {
    let res = CONTEXT.sys_user_service.update(arg.0 .0).await;
    RespVO::from_result(&res).into_response()
}

//#[get("/user/{user_id}")]
#[pre_authorize("system:user:remove")]
pub async fn remove(user_id: Path<String>) -> impl IntoResponse {
    //  let user_ids=user_id.0.split(",").collect::<Vec<&str>>();
    //  let user_cache=CONTEXT.sys_user_service.get_user_cache_by_token(&get_token()).await;
    //  match user_cache {
    //      Ok(u) => {
    //          if user_ids.contains(&u.id.as_str()) {
    //              return RespVO::<u64>::from_error_info(500,"当前用户不能删除").into_response();
    //          }
    //      }
    //      Err(_) => {}
    //  }
    //  let mut cnt=0;
    // for user_id in user_ids {
    //     let rows_affected = CONTEXT.sys_user_service.remove(user_id).await;
    //     cnt=cnt+rows_affected.unwrap_or_default();
    // }
    let rows_affected = CONTEXT.sys_user_service.remove_batch(&user_id).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功", "删除失败").into_response()
}

//#[get("/user/deptTree")]
#[pre_authorize("system:user:query")]
pub async fn get_dept_tree() -> impl IntoResponse {
    let dept_tree = CONTEXT
        .sys_dept_service
        .get_dept_tree(&get_user_name())
        .await;
    RespVO::from_result(&dept_tree).into_response()
}

//#[put("/user/authRole")]
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

//#[get("/user/authRole/{user_id}")]
#[pre_authorize("system:user:query")]
pub async fn get_auth_roles(user_id: Path<String>) -> impl IntoResponse {
    let user_id = user_id.0;
    let user = CONTEXT.sys_user_service.detail(&user_id).await;
    match user {
        Ok(user) => {
            let mut user = SysUserVO::from(user);
            user.roles = Some(
                CONTEXT
                    .sys_role_service
                    .finds_roles_by_user_id(&user_id)
                    .await
                    .unwrap_or_default(),
            );
            let roles = CONTEXT
                .sys_role_service
                .finds_roles_by_user_id(&user_id)
                .await
                .unwrap_or_default();
            let filter_roles = match get_user_name().eq(ADMIN_NAME) {
                true => roles,
                false => roles.into_iter().filter(|r| r.admin).collect::<Vec<_>>(),
            };

            let mut res = RespJson::success_info("操作成功");
            res.insert("user".to_string(), serde_json::json!(user));
            res.insert("roles".to_string(), serde_json::json!(filter_roles));
            res.into_response()
        }
        Err(e) => RespVO::<u64>::from_error_info(500, &e.to_string()).into_response(),
    }
}

//#[put("/user/resetPwd")]
#[pre_authorize("system:user:resetPwd")]
pub async fn reset_pwd(dto: Json<UserUpdateDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_user_service.update_password(dto.0).await;
    RespVO::<u64>::judge_result(res, "更新成功！", "").into_response()
}

//更改用户当前状态
//#[put("/user/changeStatus")]
#[pre_authorize("system:user:edit")]
pub async fn change_status(dto: Json<UserUpdateDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_user_service.update_status(&dto.0).await;
    RespVO::<u64>::judge_result(res, "更新成功！", "").into_response()
}
export_excel_controller!(
    "system:user:export",
    UserPageDTO,
    CONTEXT,
    sys_user_service,
    export_as_excel_bytes
);
