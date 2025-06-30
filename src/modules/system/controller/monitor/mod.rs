use axum::routing::{delete, get, post};
use axum::Router;

pub mod sys_user_online_controller;
pub mod sys_logininfor_controller;
pub mod server_controller;
pub mod sys_oper_log_controller;

pub(crate) fn build_monitor_api() -> Router {
    Router::new()
        .nest("/logininfor", logininfor_api())
        .nest("/operlog", oper_log_api())
        .nest("/online", online_api())
        .nest("/server", server_api())
        
}
fn oper_log_api() -> Router {
    Router::new()
        .route("/list", post(sys_oper_log_controller::list))
        .route("/{info_id}", delete(sys_oper_log_controller::remove))
        .route("/clean", delete(sys_oper_log_controller::clean))
        .route("/export", post(sys_oper_log_controller::export_to_excel))
}
fn logininfor_api() -> Router {
    Router::new()
        .route("/list", post(sys_logininfor_controller::list))
        .route("/{info_id}", delete(sys_logininfor_controller::remove))
        .route("/clean", delete(sys_logininfor_controller::clean))
        .route("/export", post(sys_logininfor_controller::export_to_excel))
}
fn online_api() -> Router {
    Router::new()
        .route("/list", get(sys_user_online_controller::list))
        .route("/{token_id}", delete(sys_user_online_controller::force_logout))
}
fn server_api() -> Router {
    Router::new()
        .route("/", get(server_controller::server_info))
}