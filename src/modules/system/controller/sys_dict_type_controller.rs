use crate::config::global_constants::STATUS_NORMAL;
use crate::context::CONTEXT;
use  crate::system::domain::dto::{DictTypeAddDTO, DictTypePageDTO, DictTypeUpdateDTO};
use  crate::system::domain::mapper::sys_dict_type::SysDictType;
use crate::{export_excel_controller, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

//#[get("/dict/type/list")]
#[pre_authorize("system:dict:list")]
pub async fn list(page: Json<DictTypePageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_dict_type_service.page(&page.0).await;
    PageVO::from_result(&data).into_response()
}


//#[get("/dict/type/optionselect")]
#[pre_authorize("system:dict:query")]
pub async fn optionselect() -> impl IntoResponse {
    let data = CONTEXT.sys_dict_type_service.finds_all().await;
    RespVO::from_result(&data).into_response()
}

//#[get("/dict/type/{dict_type_id}")]
#[pre_authorize("system:dict:query")]
pub async fn detail(dict_type_id: Path<String>) -> impl IntoResponse {
    let dict_type_id = dict_type_id.0;
    let dict_type_vo = CONTEXT.sys_dict_type_service.detail(&dict_type_id).await;
    RespVO::from_result(&dict_type_vo).into_response()
}

//#[post("/dict/type")]
#[pre_authorize("system:dict:add")]
pub async fn add(arg: axum_valid::Valid<Json<DictTypeAddDTO>>) -> impl IntoResponse {
    let mut data = SysDictType::from(arg.0.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.dict_name.is_none() {
        return RespVO::<u64>::from_error_info(500, "字典名字不能为空!").into_response();
    }
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let data = CONTEXT.sys_dict_type_service.add(&data).await;
    RespVO::from_result(&data).into_response()
}

//#[put("/dict/type")]
#[pre_authorize("system:dict:edit")]
pub async fn update(arg: axum_valid::Valid<Json<DictTypeUpdateDTO>>) -> impl IntoResponse {
    let mut data = SysDictType::from(arg.0.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let data = CONTEXT.sys_dict_type_service.update(data).await;
    RespVO::from_result(&data).into_response()
}

//#[delete("/dict/type/{dict_type_id}")]
#[pre_authorize("system:dict:remove")]
pub async fn remove(dict_type_id: Path<String>) -> impl IntoResponse {
    let dict_type_id = dict_type_id.0;
    let data = CONTEXT.sys_dict_type_service
        .remove_batch(&dict_type_id).await;
    RespVO::from_result(&data).into_response()
}

export_excel_controller!(
    "system:dictType:export",
    DictTypePageDTO,
    CONTEXT,
    sys_dict_type_service,
    export_as_excel_bytes
);




