use crate::context::CONTEXT;
use crate::system::domain::dto::{
    RoleAddDTO, RoleAuthUserPageDTO, RolePageDTO, RoleUpdateDTO, UserRoleDTO, UsersRoleDTO,
};
use crate::system::domain::mapper::sys_role::SysRole;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::SysRoleVO;
use crate::{export_excel_controller, PageVO, RespJson, RespVO};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use serde_json::json;

//#[get("/role/list")]
#[pre_authorize("system:role:list")]
pub async fn list(dto: Json<RolePageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_service.page(&dto.0).await;
    PageVO::from_result(&vo).into_response()
}

//#[get("/role/{role_id}")]
#[pre_authorize("system:role:query")]
pub async fn detail(role_id: Path<String>) -> impl IntoResponse {
    let role_id = role_id.0;
    let role_vo = CONTEXT
        .sys_role_service
        .detail(&role_id)
        .await
        .map(|role| SysRoleVO::from(role));
    RespVO::from_result(&role_vo).into_response()
}

//#[post("/role")]
#[pre_authorize("system:role:add")]
pub async fn add(arg: crate::ValidatedForm<RoleAddDTO>) -> impl IntoResponse {
    let arg = arg.0;
    let menu_ids = arg.menu_ids.clone().unwrap();
    let mut data = SysRole::from(arg);

    data.create_by = Some(crate::web_data::get_user_name());
    let vo = CONTEXT.sys_role_service.add(data, menu_ids).await;
    return RespVO::from_result(&vo).into_response();
}

//#[put("/role")]
#[pre_authorize("system:role:edit")]
pub async fn update(arg: crate::ValidatedForm<RoleUpdateDTO>) -> impl IntoResponse {
    let arg = arg.0;
    let menu_ids = arg.menu_ids.clone().unwrap();
    let mut data = SysRole::from(arg);
    data.update_by = Some(crate::web_data::get_user_name());
    let vo = CONTEXT.sys_role_service.update(data, menu_ids).await;
    RespVO::from_result(&vo).into_response()
}

//#[delete("/role/{role_id}")]
#[pre_authorize("system:role:remove")]
pub async fn remove(role_id: Path<String>) -> impl IntoResponse {
    let role_id = role_id.0;
    let rows_affected = CONTEXT.sys_role_service.remove_batch(&role_id).await;
    RespVO::<u64>::judge_result(rows_affected, "", "更新失败！").into_response()
}

//已分配此角色的用户
//#[get("/role/authUser/allocatedList")]
#[pre_authorize("system:role:query")]
pub async fn allocated_user_list(arg: Query<RoleAuthUserPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT
        .sys_role_service
        .allocated_user_list_page(&arg.0)
        .await;
    PageVO::from_result(&vo).into_response()
}

//未分配此角色的用户
//#[get("/role/authUser/unallocatedList")]
#[pre_authorize("system:role:query")]
pub async fn unallocated_user_list(arg: Query<RoleAuthUserPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT
        .sys_role_service
        .unallocated_user_list_page(&arg.0)
        .await;
    PageVO::from_result(&vo).into_response()
}

//取消对某个用户授权
//#[put("/role/authUser/cancel")]
#[pre_authorize("system:role:query")]
pub async fn cancel_user(arg: Json<UserRoleDTO>) -> impl IntoResponse {
    let rows_affected = CONTEXT
        .sys_user_role_service
        .remove(&SysUserRole::from(arg.0))
        .await;
    RespVO::<u64>::judge_result(rows_affected, "取消授权成功。", "取消授权失败！").into_response()
}

//对多个用户进行授权
//#[put("/role/authUser/selectAll")]
#[pre_authorize("system:role:query")]
pub async fn auth_user_all(arg: Query<UsersRoleDTO>) -> impl IntoResponse {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let rows_affected = CONTEXT
        .sys_user_role_service
        .add_users_role(&arg.0.role_id, &user_ids)
        .await;
    RespVO::<u64>::judge_result(rows_affected, "批量授权成功。", "批量授权失败！").into_response()
}
//对多个用户进行授权
//#[put("/role/authUser/cancelAll")]
#[pre_authorize("system:role:query")]
pub async fn cancel_user_all(arg: Query<UsersRoleDTO>) -> impl IntoResponse {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let rows_affected = CONTEXT
        .sys_user_role_service
        .remove_users_role(&arg.0.role_id, &user_ids)
        .await;
    RespVO::<u64>::judge_result(rows_affected, "批量取消授权成功。", "批量取消授权失败！")
        .into_response()
}

//#[put("/role/changeStatus")]
#[pre_authorize("system:role:edit")]
pub async fn change_status(arg: Json<RoleUpdateDTO>) -> impl IntoResponse {
    //  roleService.check_role_allowed(role);  todo
    //         roleService.check_role_data_scope(role.getRoleId()); todo
    //         role.setUpdateBy(getUsername());
    let mut data = SysRole::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let res = CONTEXT.sys_role_service.update(data, vec![]).await;
    RespVO::from_result(&res).into_response()
}

#[pre_authorize("system:role:query")]
pub async fn get_dept_tree_by_role_id(role_id: Path<String>) -> impl IntoResponse {
    let role_id = role_id.0;
    let role = CONTEXT.sys_role_service.detail(&role_id).await;
    let mut json = RespJson::success();
    match role {
        Ok(r) => {
            let dept_ids = CONTEXT
                .sys_dept_service
                .select_dept_list_by_role_id(&role_id, r.dept_check_strictly.eq(&Some('1')))
                .await;
            json.insert(
                "checkedKeys".to_string(),
                json!(dept_ids.unwrap_or_default()),
            );
            let depts = CONTEXT.sys_dept_service.get_dept_tree("").await;

            json.insert("depts".to_string(), json!(depts.unwrap_or_default()));
        }
        Err(_) => {}
    }
    json.into_response()
}

#[pre_authorize("system:role:edit")]
pub async fn data_scope(dto: Json<RoleUpdateDTO>) -> impl IntoResponse {
    let dept_ids = dto.0.dept_ids.clone().unwrap_or_default();
    let role = SysRole::from(dto.0);

    let r = CONTEXT
        .sys_role_service
        .auth_data_scope(&role, &dept_ids)
        .await;
    RespVO::from_result(&r).into_response()
}

export_excel_controller!(
    "system:role:export",
    RolePageDTO,
    CONTEXT,
    sys_role_service,
    export_as_excel_bytes
);
