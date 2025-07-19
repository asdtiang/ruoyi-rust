use crate::config::global_constants::ADMIN_NAME;
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
#[pre_authorize(user)]
pub async fn routers() -> impl IntoResponse {
    let user_cache = CONTEXT.sys_user_service.get_user_cache_by_token(user.login_user_key).await;
    let data = CONTEXT.sys_menu_service.get_routers(&user_cache.unwrap()).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:query",user)]
pub async fn detail(menu_id: Path<String>) -> impl IntoResponse {
    let menu_id = menu_id.0;
    let menu_vo = CONTEXT.sys_menu_service.detail(&menu_id).await.map(|m|SysMenuVO::from(m));
    RespVO::from_result(&menu_vo).into_response()
}


#[pre_authorize("system:menu:add",user)]
pub async fn add(dto: crate::ValidatedForm<MenuAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user, SysMenu);
    if data.path.is_none() {
        data.path = Some("".to_string());
    }
    let data = CONTEXT.sys_menu_service.add(data).await;
    let _=   CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:edit",user)]
pub async fn update(dto: crate::ValidatedForm<MenuUpdateDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user, SysMenu);
    let cnt = CONTEXT.sys_menu_service.update(data).await;
    RespVO::from_result(&cnt).into_response()
}


#[pre_authorize("system:menu:remove",user)]
pub async fn remove(menu_id: Path<String>) -> impl IntoResponse {
    let menu_id = menu_id.0;
    let data = CONTEXT.sys_menu_service
        .remove(&menu_id).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:menu:query",user)]
pub async fn treeselect() -> impl IntoResponse {
    let menu_select = CONTEXT.sys_menu_service.tree_select(&user.login_user_key).await;
    RespVO::from_result(&menu_select).into_response()
}


#[pre_authorize("system:menu:query",user)]
pub async fn role_menu_treeselect( role_id: Path<String>) -> impl IntoResponse {
    let role_id = role_id.0;
    let user_cache = CONTEXT.sys_user_service.get_user_cache_by_token(user.login_user_key).await;

    let user_cache = if user_cache.is_ok(){
        user_cache.unwrap()
    }else{
        return   RespVO::from_result(&user_cache).into_response();
    };
    let menus = if user_cache.user_name == ADMIN_NAME {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(&user_cache.id).await
    };

    let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(menus.unwrap_or_default()).unwrap();
    let menu_select = CONTEXT.sys_menu_service.build_tree_left_id_label(&menu_tree);

    let checked_keys = CONTEXT.sys_role_menu_service.select_by_role_id(&role_id).await.unwrap().into_iter().map(|m| m.menu_id.unwrap()).collect::<Vec<_>>();

    let mut res = RespJson::success();
    res.insert("checkedKeys".to_string(), serde_json::json!(checked_keys));
    res.insert("menus".to_string(), serde_json::json!(menu_select.unwrap()));
    res.into_response()
}