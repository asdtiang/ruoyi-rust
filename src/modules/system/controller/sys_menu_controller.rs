use crate::context::CONTEXT;
use crate::system::domain::dto::{MenuAddDTO, MenuPageDTO, MenuUpdateDTO};
use crate::system::domain::mapper::sys_menu::SysMenu;
use crate::system::domain::vo::SysMenuVO;
use crate::{add_marco, RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;


#[pre_authorize("system:menu:list")]
pub async fn list_all(dto: Option<Json<MenuPageDTO>>) -> impl IntoResponse {
    let dto=if dto.is_some() {dto.unwrap().0} else{ MenuPageDTO::default()};
    let data = CONTEXT.sys_menu_service.query_menu(&dto).await;
    RespVO::from_result(&data).into_response()
}


//菜单栏生成
#[pre_authorize(user_cache)]
pub async fn routers() -> impl IntoResponse {
    let data = CONTEXT.sys_menu_service.get_routers(&user_cache).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:query", user_cache)]
pub async fn detail(menu_id: Path<String>) -> impl IntoResponse {
    let menu_id = menu_id.0;
    let menu_vo = CONTEXT.sys_menu_service.detail(&menu_id).await.map(|m|SysMenuVO::from(m));
    RespVO::from_result(&menu_vo).into_response()
}


#[pre_authorize("system:menu:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<MenuAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, SysMenu);
    if data.path.is_none() {
        data.path = Some("".to_string());
    }
    let data = CONTEXT.sys_menu_service.add(data).await;
    let _=   CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<MenuUpdateDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, SysMenu);
    let cnt = CONTEXT.sys_menu_service.update(data).await;
    RespVO::from_result(&cnt).into_response()
}


#[pre_authorize("system:menu:remove", user_cache)]
pub async fn remove(menu_id: Path<String>) -> impl IntoResponse {
    let menu_id = menu_id.0;
    let data = CONTEXT.sys_menu_service
        .remove(&menu_id).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:query", user_cache)]
pub async fn treeselect() -> impl IntoResponse {
    let menu_select = CONTEXT.sys_menu_service.tree_select(&user_cache).await;
    RespVO::from_result(&menu_select).into_response()
}


#[pre_authorize("system:menu:query", user_cache)]
pub async fn role_menu_treeselect( role_id: Path<String>) -> impl IntoResponse {
    let role_id = role_id.0;

    let menus = if user_cache.is_admin() {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(&user_cache.user_id).await
    };

    let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(menus.unwrap_or_default()).unwrap();
    let menu_select = CONTEXT.sys_menu_service.build_tree_left_id_label(&menu_tree);

    let checked_keys = CONTEXT.sys_role_menu_service.select_by_role_id(&role_id).await.unwrap().into_iter().map(|m| m.menu_id.unwrap()).collect::<Vec<_>>();

    let mut res = RespJson::success();
    res.insert("checkedKeys".to_string(), serde_json::json!(checked_keys));
    res.insert("menus".to_string(), serde_json::json!(menu_select.unwrap()));
    res.into_response()
}