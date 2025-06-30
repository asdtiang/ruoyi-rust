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
pub mod monitor;
pub mod common_controller;

use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use crate::token_auth::middleware::log_write;

pub(crate) fn build_auth_api() -> Router {
    Router::new()
        .route("/captchaImage", get(sys_auth_controller::captcha))
        .route("/login", post(sys_auth_controller::login))
        .route("/logout", post(sys_auth_controller::logout))
        .route("/getInfo", get(sys_auth_controller::info))
        .route("/getRouters", get(sys_menu_controller::routers))
}
pub(crate) fn build_common_api() -> Router {
    Router::new()
        .route("/upload", post(common_controller::upload))
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
        .route("/", post(sys_user_controller::add))
        .route("/", put(sys_user_controller::update))
        .route("/{user_id}", delete(sys_user_controller::remove))
        .route("/deptTree", get(sys_user_controller::get_dept_tree))
        .route("/changeStatus", put(sys_user_controller::change_status))
        .route("/resetPwd", put(sys_user_controller::reset_pwd))
        .route("/authRole", put(sys_user_controller::set_auth_roles))
        .route(
            "/authRole/{user_id}",
            get(sys_user_controller::get_auth_roles),
        )
        //profile
        .route("/profile", get(sys_profile_controller::profile))
        .route("/profile", put(sys_profile_controller::profile))
        .route("/updatePwd", put(sys_profile_controller::update_pwd))
        .route("/export", post(sys_user_controller::export_to_excel))
}

fn menu_api() -> Router {
    Router::new()
        .route("/list", post(sys_menu_controller::list_all))
        .route("/{menu_id}", get(sys_menu_controller::detail))
        .route("/", post(sys_menu_controller::add))
        .route("/", put(sys_menu_controller::update))
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
        .route(
            "/list/exclude/{dept_id}",
            get(sys_dept_controller::exclude_child),
        )
        .route("/{dept_id}", get(sys_dept_controller::detail))
        .route("/", post(sys_dept_controller::add))
        .route("/", put(sys_dept_controller::update))
        .route("/{dept_id}", delete(sys_dept_controller::remove))
}

fn role_api() -> Router {
    Router::new()
        .route("/list", post(sys_role_controller::list))
        .route("/{role_id}", get(sys_role_controller::detail))
        .route("/", post(sys_role_controller::add))
        .route("/", put(sys_role_controller::update))
        .route("/{role_id}", delete(sys_role_controller::remove))
        .route(
            "/authUser/allocatedList",
            get(sys_role_controller::allocated_user_list),
        )
        .route(
            "/authUser/unallocatedList",
            get(sys_role_controller::unallocated_user_list),
        )
        .route("/authUser/cancel", put(sys_role_controller::cancel_user))
        .route(
            "/authUser/selectAll",
            put(sys_role_controller::auth_user_all),
        )
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
        .route("/", post(sys_dict_type_controller::add))
        .route("/", put(sys_dict_type_controller::update))
        .route("/{dict_type_id}", delete(sys_dict_type_controller::remove))
        .route("/export", post(sys_dict_type_controller::export_to_excel))
}

fn dict_data_api() -> Router {
    Router::new()
        .route("/list", post(sys_dict_data_controller::list))
        .route("/{dict_data_id}", get(sys_dict_data_controller::detail))
        .route("/", post(sys_dict_data_controller::add))
        .route("/", put(sys_dict_data_controller::update))
        .route("/{dict_data_id}", delete(sys_dict_data_controller::remove))
        .route(
            "/type/{dict_type}",
            get(sys_dict_data_controller::get_by_dict_type),
        )
        .route("/export", post(sys_dict_data_controller::export_to_excel))
}

fn post_api() -> Router {
    Router::new()
        .route("/list", post(sys_post_controller::list))
        .route("/{post_id}", get(sys_post_controller::detail))
        .route("/", post(sys_post_controller::add))
        .route("/", put(sys_post_controller::update))
        .route("/{post_id}", delete(sys_post_controller::remove))
        .route("/export", post(sys_post_controller::export_to_excel))
}

fn config_api() -> Router {
    Router::new()
        .route("/list", post(sys_config_controller::list).route_layer(middleware::from_fn(log_write)))
        .route("/", post(sys_config_controller::add).route_layer(middleware::from_fn(log_write)))
        .route("/{config_id}", get(sys_config_controller::detail))
        .route("/", put(sys_config_controller::update))
        .route("/{config_id}", delete(sys_config_controller::remove))
        .route(
            "/refreshCache",
            delete(sys_config_controller::refresh_cache),
        )
        .route("/export", post(sys_config_controller::export_to_excel))
}

fn notice_api() -> Router {
    Router::new()
        .route("/list", post(sys_notice_controller::list))
        .route("/{notice_id}", get(sys_notice_controller::detail))
        .route("/", post(sys_notice_controller::add))
        .route("/", put(sys_notice_controller::update))
        .route("/{notice_id}", delete(sys_notice_controller::remove))
        .route("/export", post(sys_notice_controller::export_to_excel))
}
