use crate::context::CONTEXT;
use crate::system::domain::mapper::sys_oper_log::SysOperLog;
use axum::extract::{ OriginalUri, State};
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use crate::{UserCache};
use crate::web::extractors::ip::ClientIp;

///操作日志登记
pub async fn log_write_state(
    ori_uri: OriginalUri,
    ClientIp(ip): ClientIp,
    State(state): State<crate::OperState>,
    user_cache: UserCache,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let request_method: Option<String> = Some(req.method().to_string());

    let (parts, body) = req.into_parts();

    let url = ori_uri.path_and_query().map(|x| x.to_string());

    let bytes = get_body_bytes(body).await?;
    let oper_param = std::str::from_utf8(&bytes).ok().map(|x| x.to_string());
    let req = Request::from_parts(parts, Body::from(bytes));

    let start = DateTime::now();
    let res = next.run(req).await;
    let cost_time = (DateTime::now().unix_timestamp_millis() - start.unix_timestamp_millis()) as u64;

    // 检查响应状态，如果是错误则记录日志
    let status = res.status();
    if status.is_server_error() {
        let backtrace = std::backtrace::Backtrace::capture();
        if backtrace.status() == std::backtrace::BacktraceStatus::Disabled {
            log::error!("[HTTP {}] Server error for request {} {}", status.as_u16(), request_method.as_deref().unwrap_or(""), url.as_deref().unwrap_or(""));
            log::warn!("[BACKTRACE] Backtrace is disabled. To enable detailed stack trace, set environment variable: RUST_BACKTRACE=1");
        } else {
            log::error!("[HTTP {}] Server error for request {} {}\nBacktrace:\n{}", status.as_u16(), request_method.as_deref().unwrap_or(""), url.as_deref().unwrap_or(""), backtrace);
        }
    } else if status.is_client_error() {
        log::warn!("[HTTP {}] Client error for request {} {}", status.as_u16(), request_method.as_deref().unwrap_or(""), url.as_deref().unwrap_or(""));
    }

    let (parts, body) = res.into_parts();
    let bytes = get_body_bytes(body).await?;
    let json_result = std::str::from_utf8(&bytes).ok().map(|x| x.to_string());

    let sys_oper_log = SysOperLog {
        oper_id: ObjectId::new().to_string().into(),
        title: state.title.into(),
        business_type: Some(state.business_type),
        method:state.path.into(),
        request_method,
        operator_type: 0.into(),//todo
        oper_name: user_cache.user_name.into(),
        dept_name: user_cache.dept_name.into(),
        oper_url: url,
        oper_ip: Some(ip),
        oper_location: None,
        oper_param,
        json_result,
        status: None,
        error_msg: None,
        oper_time: start.into(),
        cost_time: cost_time.into(),
    };
    let _ = CONTEXT.sys_oper_log_service.add_async(&sys_oper_log).await;
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}


async fn get_body_bytes<B>(body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, format!("failed to read body: {err}")));
        }
    };
    Ok(bytes)
}
