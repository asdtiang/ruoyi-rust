use crate::context::CONTEXT;
use  crate::system::domain::vo::SysOperLogVO;
use crate::{export_excel_controller, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use rbatis::Page;
use crate::system::domain::dto::OperLogPageDTO;

//#[get("/logininfor/list")]
#[pre_authorize("monitor:operlog:list")]
pub async fn list(dto: Json<OperLogPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_oper_log_service.page(&dto.0).await;
    let data=data.map(|l|Page::<SysOperLogVO>::from(l));
    PageVO::from_result(&data).into_response()
}


export_excel_controller!(
    "system:operLog:export",
    OperLogPageDTO,
    CONTEXT,
    sys_oper_log_service,
    export_as_excel_bytes
);






//#[delete("/logininfor/{oper_id}")]
#[pre_authorize("monitor:operlog:remove")]
pub async fn remove(oper_id: Path<String>) -> impl IntoResponse {
    let oper_id = oper_id.0;
    let rows_affected = CONTEXT
        .sys_oper_log_service
        .remove_batch(&oper_id)
        .await;
    RespVO::<u64>::judge_result(rows_affected, "", "删除失败！").into_response()
}
#[pre_authorize("monitor:operlog:remove")]
pub async fn clean() -> impl IntoResponse {

    let rows_affected = CONTEXT .sys_oper_log_service.  clean() .await;
    RespVO::<u64>::judge_result(rows_affected, "", "清空失败！").into_response()
}