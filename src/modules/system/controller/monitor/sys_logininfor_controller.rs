use crate::context::CONTEXT;
use  crate::system::domain::dto::LogininforPageDTO;
use  crate::system::domain::vo::SysLogininforVO;
use crate::{PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use rbatis::Page;

//#[get("/logininfor/list")]
#[pre_authorize("monitor:logininfor:list")]
pub async fn list(dto: Json<LogininforPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_logininfor_service.page(&dto.0).await;
    let data=data.map(|l|Page::<SysLogininforVO>::from(l));
    PageVO::from_result(&data).into_response()
}


//#[delete("/logininfor/{info_id}")]
#[pre_authorize("monitor:logininfor:remove")]
pub async fn remove(info_id: Path<String>) -> impl IntoResponse {
    let info_id = info_id.0;
    let rows_affected = CONTEXT
        .sys_logininfor_service
        .remove(&info_id)
        .await;
    RespVO::<u64>::judge_result(&rows_affected, "", "删除失败！").into_response()
}
#[pre_authorize("monitor:logininfor:remove")]
pub async fn clean() -> impl IntoResponse {

    let rows_affected = CONTEXT .sys_logininfor_service.  clean() .await;
    RespVO::<u64>::judge_result(&rows_affected, "", "清空失败！").into_response()
}