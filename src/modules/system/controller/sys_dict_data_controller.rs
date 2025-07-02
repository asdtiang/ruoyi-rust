use crate::config::global_constants::STATUS_NORMAL;
use crate::context::CONTEXT;
use  crate::system::domain::dto::{DictDataAddDTO, DictDataPageDTO, DictDataUpdateDTO};
use  crate::system::domain::mapper::sys_dict_data::SysDictData;
use crate::{export_excel_controller, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

//#[get("/dict/data/list")]
#[pre_authorize("system:dict:list")]
pub async fn list(dto: Json<DictDataPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_dict_data_service.page(&dto.0).await;
    PageVO::from_result(&data).into_response()
}


//#[get("/dict/data/{dict_id}")]
#[pre_authorize("system:dict:query")]
pub async fn detail(dict_data_id: Path<String>) -> impl IntoResponse {
    let dict_data_id = dict_data_id.0;
    let dict_data_vo = CONTEXT.sys_dict_data_service.detail(&dict_data_id).await;
    RespVO::from_result(&dict_data_vo).into_response()
}

//#[post("/dict/data")]
#[pre_authorize("system:dict:add")]
pub async fn add(arg: axum_valid::Valid<Json<DictDataAddDTO>>) -> impl IntoResponse {
    let mut data = SysDictData::from(arg.0.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let rows_affected = CONTEXT.sys_dict_data_service.add(&data).await;
    RespVO::<u64>::judge_result(rows_affected, "", "添加失败！").into_response()
}

//#[put("/dict/data")]
#[pre_authorize("system:dict:edit")]
pub async fn update(arg: axum_valid::Valid<Json<DictDataUpdateDTO>>) -> impl IntoResponse {
    let mut data = SysDictData::from(arg.0.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let rows_affected = CONTEXT.sys_dict_data_service.update(data).await;
    RespVO::<u64>::judge_result(rows_affected, "", "更新失败！").into_response()
}

//#[delete("/dict/data/{dict_code}")]
#[pre_authorize("system:dict:remove")]
pub async fn remove(dict_code: Path<String>) -> impl IntoResponse {
    let dict_code = dict_code.0;
    let rows_affected = CONTEXT
        .sys_dict_data_service
        .remove_batch(&dict_code)
        .await;
    RespVO::<u64>::judge_result(rows_affected, "", "删除失败！").into_response()
}


//#[get("/dict/data/type/{dict_type}")]
#[pre_authorize("")]
pub async fn get_by_dict_type(dict_type: Path<String>) -> impl IntoResponse {
    let dict_type = dict_type.0;
    let dict_data_vo = CONTEXT.sys_dict_data_service.get_by_dict_type(&dict_type).await;
    RespVO::from_result(&dict_data_vo).into_response()
}


export_excel_controller!(
    "system:dictData:export",
    DictDataPageDTO,
    CONTEXT,
    sys_dict_data_service,
    export_as_excel_bytes
);


