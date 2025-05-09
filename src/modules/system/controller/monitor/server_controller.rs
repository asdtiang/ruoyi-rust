use crate::RespVO;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::system::service::hardware::get_server_info;

//#[get("/server")]
#[pre_authorize("monitor:server:list")]
pub async fn server_info() -> impl IntoResponse {
   RespVO::from_result(&Ok(get_server_info())).into_response()
}


