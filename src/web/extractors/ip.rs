use std::net::SocketAddr;
use crate::RespVO;
use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use crate::utils::ip_util::get_ip_addr;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClientIp(pub String);
impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = RespVO<u64>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let head_ip = get_ip_addr(&parts.headers);
        let ip = if head_ip.is_none() {
            if let Some(ConnectInfo(addr)) = parts.extensions.get::<ConnectInfo<SocketAddr>>() {
                addr.ip().to_string()
            }else{
                "unknown".to_string()
            }
        } else{
            head_ip.unwrap_or_default()
        };
        Ok(ClientIp(ip))
    }
}
