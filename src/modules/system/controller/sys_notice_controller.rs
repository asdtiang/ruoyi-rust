use crate::context::CONTEXT;
use crate::system::domain::dto::{NoticeAddDTO, NoticePageDTO, NoticeUpdateDTO};
use crate::system::domain::mapper::sys_notice::SysNotice;
use crate::system::domain::vo::SysNoticeVO;
use crate::{add_marco, export_excel_controller, update_marco, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use rbatis::Page;


#[pre_authorize("system*:notice:list")]
pub async fn list(dto: Json<NoticePageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_notice_service.page(&dto.0).await;
    let data = data.map(|d| Page::<SysNoticeVO>::from(d));
    PageVO::from_result(&data).into_response()
}


#[pre_authorize("system*:notice:query")]
pub async fn detail(notice_id: Path<String>) -> impl IntoResponse {
    let notice = CONTEXT.sys_notice_service.detail(&notice_id.0).await;
    RespVO::from_result(&notice).into_response()
}


#[pre_authorize("system:notice:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<NoticeAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, SysNotice);
    let res = CONTEXT.sys_notice_service.add(data).await;
    RespVO::from_result(&res).into_response()
}


#[pre_authorize("system:notice:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<NoticeUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, SysNotice);
    let res = CONTEXT.sys_notice_service.update(data).await;
    RespVO::from_result(&res).into_response()
}


#[pre_authorize("system:notice:remove")]
pub async fn remove(notice_id: Path<String>) -> impl IntoResponse {
    let rows_affected = CONTEXT.sys_notice_service.remove_batch(&notice_id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "", "删除失败！").into_response()
}

export_excel_controller!(
    "system:notice:export",
    NoticePageDTO,
    CONTEXT,
    sys_notice_service,
    export_as_excel_bytes
);
