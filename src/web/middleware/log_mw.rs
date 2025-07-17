use crate::context::CONTEXT;
use crate::system::domain::mapper::sys_oper_log::SysOperLog;
use crate::utils::address_util;
use crate::utils::ip_util::get_ip_addr;
use axum::extract::{ConnectInfo, OriginalUri};
use axum::http::HeaderMap;
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response}
};
use http_body_util::BodyExt;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use std::net::SocketAddr;

pub async fn log_write(
    ori_uri: OriginalUri,
    socket_addr: ConnectInfo<SocketAddr>,
    header_map: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let method = req.method().to_string().into();
    let (parts, body) = req.into_parts();
    let head_ip = get_ip_addr(&header_map);
    let ip = if head_ip.is_none() {
        socket_addr.0.ip().to_string().into()
    } else {
        head_ip
    };

    let url = ori_uri.path_and_query().map(|x| x.to_string());

    let address = if CONTEXT.config.address_enabled {
        match ip.clone() {
            Some(ip) => address_util::get_real_address_by_ip(&ip).await.ok(),
            None => None,
        }
    } else {
        None
    };

    let bytes = get_body_bytes(body).await?;
    let oper_param = std::str::from_utf8(&bytes).ok().map(|x| x.to_string());
    let req = Request::from_parts(parts, Body::from(bytes));

    let start = DateTime::now();
    let res = next.run(req).await;
    let cost_time =
        (DateTime::now().unix_timestamp_millis() - start.unix_timestamp_millis()) as u64;

    let (parts, body) = res.into_parts();
    let bytes = get_body_bytes(body).await?;
    let json_result = std::str::from_utf8(&bytes).ok().map(|x| x.to_string());

    let sys_oper_log = SysOperLog {
        oper_id: ObjectId::new().to_string().into(),
        title: String::new().into(),
        business_type: 0.into(),
        method,
        request_method: String::new().into(),
        operator_type: 0.into(),
        oper_name: String::new().into(),
        dept_name: String::new().into(),
        oper_url: url,
        oper_ip: ip,
        oper_location: address,
        oper_param,
        json_result,
        status: None,
        error_msg: None,
        oper_time: start.into(),
        cost_time: cost_time.into(),
    };
  let _=  CONTEXT.sys_oper_log_service.add_async(&sys_oper_log).await;

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
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read body: {err}"),
            ));
        }
    };
    Ok(bytes)
}
