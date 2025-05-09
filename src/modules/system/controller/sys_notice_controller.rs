use crate::context::CONTEXT;
use  crate::system::domain::dto::{NoticeAddDTO, NoticePageDTO, NoticeUpdateDTO};
use  crate::system::domain::vo::SysNoticeVO;
use crate::{PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use rbatis::Page;

//#[get("/notice/list")]
#[pre_authorize("system:notice:list")]
pub async fn list(dto: Json<NoticePageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_notice_service.page(&dto.0).await;
    let data = data.map(|d| Page::<SysNoticeVO>::from(d));
    PageVO::from_result(&data).into_response()
}

//#[get("/notice/{notice_id}")]
#[pre_authorize("system:notice:query")]
pub async fn detail(notice_id: Path<String>) -> impl IntoResponse {
    let notice = CONTEXT.sys_notice_service.detail(&notice_id.0).await;
    let notice = notice.map(|d| SysNoticeVO::from(d));
    RespVO::from_result(&notice).into_response()
}

//#[post("/notice")]
#[pre_authorize("system:notice:add")]
pub async fn add(dto: Json<NoticeAddDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_notice_service.add(dto.0).await;
    RespVO::from_result(&res).into_response()
}

//#[put("/notice")]
#[pre_authorize("system:notice:edit")]
pub async fn update(dto: Json<NoticeUpdateDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_notice_service.update(dto.0).await;
    RespVO::from_result(&res).into_response()
}

//#[delete("/notice/{notice_id}")]
#[pre_authorize("system:notice:remove")]
pub async fn remove(notice_id: Path<String>) -> impl IntoResponse {
    let rows_affected = CONTEXT.sys_notice_service.remove_batch(&notice_id.0).await;
    RespVO::<u64>::judge_result(&rows_affected, "", "删除失败！")
        .into_response()
}
