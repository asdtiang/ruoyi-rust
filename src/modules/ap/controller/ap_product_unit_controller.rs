use crate::ap::domain::vo::{ApProductUnitListVO, ApProductUnitVO};
use crate::{PageVO, RespVO, add_marco, export_excel_controller, update_marco};
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::ap::AP_CONTEXT;
use crate::ap::domain::dto::{ApProductUnitAddDTO, ApProductUnitPageDTO, ApProductUnitUpdateDTO};
use crate::ap::domain::mapper::ApProductUnit;
use rbatis::Page;
//查询单位列表
#[pre_authorize("ap:productUnit:list")]
pub async fn list(dto: Json<ApProductUnitPageDTO>) -> impl IntoResponse {
    let data = AP_CONTEXT.ap_product_unit_service.page(&dto.0).await;
    let data = data.map(|d| Page::<ApProductUnitListVO>::from(d));
    PageVO::from_result(&data).into_response()
}
//获取单位详细信息
#[pre_authorize("ap:productUnit:query")]
pub async fn detail(id: Path<String>) -> impl IntoResponse {
    let product_unit = AP_CONTEXT.ap_product_unit_service.detail(&id.0).await;
    let product_unit = product_unit.map(|d| ApProductUnitVO::from(d));
    RespVO::from_result(&product_unit).into_response()
}
//新增单位
#[pre_authorize("ap:productUnit:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<ApProductUnitAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, ApProductUnit);
    let res = AP_CONTEXT.ap_product_unit_service.add(data).await;
    RespVO::from_result(&res).into_response()
}
//更新单位
#[pre_authorize("ap:productUnit:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<ApProductUnitUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, ApProductUnit);
    let res = AP_CONTEXT.ap_product_unit_service.update(data).await;
    RespVO::from_result(&res).into_response()
}
//删除单位
#[pre_authorize("ap:productUnit:remove")]
pub async fn remove(id: Path<String>) -> impl IntoResponse {
    let rows_affected = AP_CONTEXT.ap_product_unit_service.remove_batch(&id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功！", "删除失败！").into_response()
}
//导出单位
export_excel_controller!(
    "ap:productUnit:export",
    ApProductUnitPageDTO,
    AP_CONTEXT,
    ap_product_unit_service,
    export_as_excel_bytes
);
