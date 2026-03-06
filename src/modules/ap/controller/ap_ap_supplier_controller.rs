use crate::ap::domain::vo::{ApSupplierListVO, ApSupplierVO};
use crate::{PageVO, RespVO, add_marco, export_excel_controller, update_marco};
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::ap::AP_CONTEXT;
use crate::ap::domain::dto::{ApSupplierAddDTO, ApSupplierPageDTO, ApSupplierUpdateDTO};
use crate::ap::domain::mapper::ApSupplier;
use rbatis::Page;
//查询供应商列表
#[pre_authorize("ap:ApSupplier:list")]
pub async fn list(dto: Json<ApSupplierPageDTO>) -> impl IntoResponse {
    let data = AP_CONTEXT.ap_ap_supplier_service.page(&dto.0).await;
    let data = data.map(|d| Page::<ApSupplierListVO>::from(d));
    PageVO::from_result(&data).into_response()
}
//获取供应商详细信息
#[pre_authorize("ap:ApSupplier:query")]
pub async fn detail(id: Path<String>) -> impl IntoResponse {
    let ap_supplier = AP_CONTEXT.ap_ap_supplier_service.detail(&id.0).await;
    let ap_supplier = ap_supplier.map(|d| ApSupplierVO::from(d));
    RespVO::from_result(&ap_supplier).into_response()
}
//新增供应商
#[pre_authorize("ap:ApSupplier:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<ApSupplierAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, ApSupplier);
    let res = AP_CONTEXT.ap_ap_supplier_service.add(data).await;
    RespVO::from_result(&res).into_response()
}
//更新供应商
#[pre_authorize("ap:ApSupplier:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<ApSupplierUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, ApSupplier);
    let res = AP_CONTEXT.ap_ap_supplier_service.update(data).await;
    RespVO::from_result(&res).into_response()
}
//删除供应商
#[pre_authorize("ap:ApSupplier:remove")]
pub async fn remove(id: Path<String>) -> impl IntoResponse {
    let rows_affected = AP_CONTEXT.ap_ap_supplier_service.remove_batch(&id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功！", "删除失败！").into_response()
}
//导出供应商
export_excel_controller!(
    "ap:ApSupplier:export",
    ApSupplierPageDTO,
    AP_CONTEXT,
    ap_ap_supplier_service,
    export_as_excel_bytes
);
