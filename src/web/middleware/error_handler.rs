use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use crate::error::Error;

/// 全局错误处理中间件
/// 捕获所有未处理的错误并记录日志
pub async fn handle_error(err: Box<dyn std::error::Error>) -> Response {
    // 记录完整的错误信息，包括调用堆栈
    log::error!(
        "[UNHANDLED ERROR] Type: {}, Message: {}",
        std::any::type_name_of_val(&*err),
        err
    );

    // 尝试获取更多上下文信息
    if let Some(source) = err.source() {
        log::error!("[UNHANDLED ERROR] Caused by: {}", source);
    }

    // 返回 500 错误响应
    let error_msg = if err.is::<Error>() {
        err.to_string()
    } else {
        "Internal server error".to_string()
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        axum::Json(serde_json::json!({
            "code": 500,
            "msg": error_msg,
            "data": null
        }))
    ).into_response()
}
