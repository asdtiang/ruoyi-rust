use crate::ap::domain::vo::{ApProductBrandListVO, ApProductBrandVO};
use crate::{PageVO, RespVO, add_marco, export_excel_controller, update_marco};
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::ap::AP_CONTEXT;
use crate::ap::domain::dto::{ApProductBrandAddDTO, ApProductBrandPageDTO, ApProductBrandUpdateDTO};
use crate::ap::domain::mapper::ApProductBrand;
use rbatis::Page;
//查询品牌库列表
#[pre_authorize("ap:ProductBrand:list")]
pub async fn list(dto: Json<ApProductBrandPageDTO>) -> impl IntoResponse {
    let data = AP_CONTEXT.ap_product_brand_service.page(&dto.0).await;
    let data = data.map(|d| Page::<ApProductBrandListVO>::from(d));
    PageVO::from_result(&data).into_response()
}
//获取品牌库详细信息
#[pre_authorize("ap:ProductBrand:query")]
pub async fn detail(id: Path<String>) -> impl IntoResponse {
    let product_brand = AP_CONTEXT.ap_product_brand_service.detail(&id.0).await;
    let product_brand = product_brand.map(|d| ApProductBrandVO::from(d));
    RespVO::from_result(&product_brand).into_response()
}
//新增品牌库
#[pre_authorize("ap:ProductBrand:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<ApProductBrandAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, ApProductBrand);
    let res = AP_CONTEXT.ap_product_brand_service.add(data).await;
    RespVO::from_result(&res).into_response()
}
//更新品牌库
#[pre_authorize("ap:ProductBrand:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<ApProductBrandUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, ApProductBrand);
    let res = AP_CONTEXT.ap_product_brand_service.update(data).await;
    RespVO::from_result(&res).into_response()
}
//删除品牌库
#[pre_authorize("ap:ProductBrand:remove")]
pub async fn remove(id: Path<String>) -> impl IntoResponse {
    let rows_affected = AP_CONTEXT.ap_product_brand_service.remove_batch(&id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功！", "删除失败！").into_response()
}
//导出品牌库
export_excel_controller!(
    "ap:ProductBrand:export",
    ApProductBrandPageDTO,
    AP_CONTEXT,
    ap_product_brand_service,
    export_as_excel_bytes
);
