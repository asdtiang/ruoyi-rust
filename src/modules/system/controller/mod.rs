pub mod common_controller;
pub mod monitor;
pub mod sys_auth_controller;
pub mod sys_config_controller;
pub mod sys_dept_controller;
pub mod sys_dict_data_controller;
pub mod sys_dict_type_controller;
pub mod sys_menu_controller;
pub mod sys_notice_controller;
pub mod sys_post_controller;
pub mod sys_profile_controller;
pub mod sys_role_controller;
pub mod sys_user_controller;

use axum::routing::{delete, get, post, put};
use axum::Router;
use crate::router_log;

pub(crate) fn build_auth_api() -> Router {
    Router::new()
        .route("/captchaImage", get(sys_auth_controller::captcha))
        .route("/login", post(sys_auth_controller::login))
        .route("/logout", post(sys_auth_controller::logout))
        .route("/getInfo", get(sys_auth_controller::info))
        .route("/getRouters", get(sys_menu_controller::routers))
}
pub(crate) fn build_common_api() -> Router {
    Router::new().route("/upload", post(common_controller::upload))
}
pub(crate) fn build_system_api() -> Router {
    Router::new()
        .nest("/user", user_api())
        .nest("/menu", menu_api())
        .nest("/dept", dept_api())
        .nest("/dict/type", dict_type_api())
        .nest("/dict/data", dict_data_api())
        .nest("/role", role_api())
        .nest("/post", post_api())
        .nest("/config", config_api())
        .nest("/notice", notice_api())
}

fn user_api() -> Router {
    Router::new()
        .route("/list", post(sys_user_controller::list))
        //fixme 根据文档，为什么没有办法匹配/user/
        .route("/", get(sys_user_controller::detail))
        .route("/{user_id}", get(sys_user_controller::detail))
        .route("/", router_log!(post, sys_user_controller::add, "用户管理", INSERT))
        .route("/", router_log!(put, sys_user_controller::update, "用户管理", UPDATE))
        .route("/{user_id}", delete(sys_user_controller::remove))
        .route("/deptTree", get(sys_user_controller::get_dept_tree))
        .route("/changeStatus", put(sys_user_controller::change_status))
        .route("/resetPwd", put(sys_user_controller::reset_pwd))
        .route("/authRole", put(sys_user_controller::set_auth_roles))
        .route("/authRole/{user_id}", get(sys_user_controller::get_auth_roles))
        .route("/export", post(sys_user_controller::export_to_excel))
        //profile
        .route("/profile", get(sys_profile_controller::profile))
        .route("/profile", put(sys_profile_controller::update_profile))
        .route("/profile/updatePwd", put(sys_profile_controller::update_pwd))

}

fn menu_api() -> Router {
    Router::new()
        .route("/list", post(sys_menu_controller::list_all))
        .route("/{menu_id}", get(sys_menu_controller::detail))
        .route("/", router_log!(post, sys_menu_controller::add, "菜单管理", INSERT))
        .route("/", router_log!(put, sys_menu_controller::update, "菜单管理", UPDATE))
        .route("/{menu_id}", delete(sys_menu_controller::remove))
        .route("/treeselect", get(sys_menu_controller::treeselect))
        .route(
            "/roleMenuTreeselect/{role_id}",
            get(sys_menu_controller::role_menu_treeselect),
        )
}

fn dept_api() -> Router {
    Router::new()
        .route("/list", post(sys_dept_controller::list))
        .route("/list/exclude/{dept_id}", get(sys_dept_controller::exclude_child))
        .route("/{dept_id}", get(sys_dept_controller::detail))
        .route("/", router_log!(post, sys_dept_controller::add, "部门管理", INSERT))
        .route("/", router_log!(put, sys_dept_controller::update, "部门管理", UPDATE))
        .route("/{dept_id}", delete(sys_dept_controller::remove))
}

fn role_api() -> Router {
    Router::new()
        .route("/list", post(sys_role_controller::list))
        .route("/{role_id}", get(sys_role_controller::detail))
        .route("/", router_log!(post, sys_role_controller::add, "角色管理", INSERT))
        .route("/", router_log!(put, sys_role_controller::update, "角色管理", UPDATE))
        .route("/{role_id}", delete(sys_role_controller::remove))
        .route("/authUser/allocatedList", get(sys_role_controller::allocated_user_list))
        .route(
            "/authUser/unallocatedList",
            get(sys_role_controller::unallocated_user_list),
        )
        .route("/authUser/cancel", put(sys_role_controller::cancel_user))
        .route("/authUser/selectAll", put(sys_role_controller::auth_user_all))
        .route("/changeStatus", put(sys_role_controller::change_status))
        .route(
            "/deptTree/{role_id}",
            get(sys_role_controller::get_dept_tree_by_role_id),
        )
        .route("/dataScope", put(sys_role_controller::data_scope))
        .route("/export", post(sys_role_controller::export_to_excel))
}

fn dict_type_api() -> Router {
    Router::new()
        .route("/list", post(sys_dict_type_controller::list))
        .route("/optionselect", get(sys_dict_type_controller::optionselect))
        .route("/{dict_type_id}", get(sys_dict_type_controller::detail))
        .route("/", router_log!(post, sys_dict_type_controller::add, "字典类型", INSERT))
        .route("/", router_log!(put, sys_dict_type_controller::update, "字典类型", UPDATE))
        .route("/{dict_type_id}", delete(sys_dict_type_controller::remove))
        .route("/export", post(sys_dict_type_controller::export_to_excel))
}

fn dict_data_api() -> Router {
    Router::new()
        .route("/list", post(sys_dict_data_controller::list))
        .route("/{dict_data_id}", get(sys_dict_data_controller::detail))
        .route("/", router_log!(post, sys_dict_data_controller::add, "字典数据", INSERT))
        .route("/", router_log!(put, sys_dict_data_controller::update, "字典数据", UPDATE))
        .route("/{dict_data_id}", delete(sys_dict_data_controller::remove))
        .route("/type/{dict_type}", get(sys_dict_data_controller::get_by_dict_type))
        .route("/export", post(sys_dict_data_controller::export_to_excel))
}

fn post_api() -> Router {
    Router::new()
        .route("/list", post(sys_post_controller::list))
        .route("/{post_id}", get(sys_post_controller::detail))
        .route("/", router_log!(post, sys_post_controller::add, "岗位管理", INSERT))
        .route("/", router_log!(put, sys_post_controller::update, "岗位管理", UPDATE))
        .route("/{post_id}", delete(sys_post_controller::remove))
        .route("/export", post(sys_post_controller::export_to_excel))
}

fn config_api() -> Router {
    Router::new()
        .route("/list", post(sys_config_controller::list))
        .route("/", router_log!(post, sys_config_controller::add, "参数设置", INSERT))
        .route("/", router_log!(put, sys_config_controller::update, "参数设置", UPDATE))
        .route("/{config_id}", get(sys_config_controller::detail))
        //fixme 根据文档，为什么没有办法匹配/user/
        .route("/refreshCache/", delete(sys_config_controller::refresh_cache))
        .route("/{config_id}", delete(sys_config_controller::remove))
        .route("/export", post(sys_config_controller::export_to_excel))
}

fn notice_api() -> Router {
    Router::new()
        .route("/list", post(sys_notice_controller::list))
        .route("/{notice_id}", get(sys_notice_controller::detail))
        .route("/", router_log!(post, sys_notice_controller::add, "通知公告", INSERT))
        .route("/", router_log!(put, sys_notice_controller::update, "通知公告", UPDATE))
        .route("/{notice_id}", delete(sys_notice_controller::remove))
        .route("/export", post(sys_notice_controller::export_to_excel))
}
