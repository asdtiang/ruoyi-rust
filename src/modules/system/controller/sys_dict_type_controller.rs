use crate::config::global_constants::STATUS_NORMAL;
use crate::context::CONTEXT;
use crate::system::domain::dto::{DictTypeAddDTO, DictTypePageDTO, DictTypeUpdateDTO};
use crate::system::domain::mapper::sys_dict_type::SysDictType;
use crate::{add_marco, export_excel_controller, update_marco, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;


#[pre_authorize("system:dict:list")]
pub async fn list(page: Json<DictTypePageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_dict_type_service.page(&page.0).await;
    PageVO::from_result(&data).into_response()
}


#[pre_authorize("system:dict:query", user_cache)]
pub async fn optionselect() -> impl IntoResponse {
    let data = CONTEXT.sys_dict_type_service.finds_all().await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:dict:query", user_cache)]
pub async fn detail(dict_type_id: Path<String>) -> impl IntoResponse {
    let dict_type_id = dict_type_id.0;
    let dict_type_vo = CONTEXT.sys_dict_type_service.detail(&dict_type_id).await;
    RespVO::from_result(&dict_type_vo).into_response()
}


#[pre_authorize("system:dict:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<DictTypeAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, SysDictType);
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_dict_type_service.add(&data).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:dict:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<DictTypeUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, SysDictType);
    let data = CONTEXT.sys_dict_type_service.update(data).await;
    RespVO::from_result(&data).into_response()
}


#[pre_authorize("system:dict:remove")]
pub async fn remove(dict_type_id: Path<String>) -> impl IntoResponse {
    let dict_type_id = dict_type_id.0;
    let data = CONTEXT.sys_dict_type_service.remove_batch(&dict_type_id).await;
    RespVO::from_result(&data).into_response()
}

export_excel_controller!(
    "system:dictType:export",
    DictTypePageDTO,
    CONTEXT,
    sys_dict_type_service,
    export_as_excel_bytes
);
