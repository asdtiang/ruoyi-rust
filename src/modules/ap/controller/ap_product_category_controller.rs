use crate::ap::domain::vo::{ApProductCategoryListVO, ApProductCategoryVO};
use crate::{PageVO, RespVO, add_marco, export_excel_controller, update_marco};
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use macros::pre_authorize;
use crate::ap::AP_CONTEXT;
use crate::ap::domain::dto::{ApProductCategoryAddDTO, ApProductCategoryPageDTO, ApProductCategoryUpdateDTO};
use crate::ap::domain::mapper::ApProductCategory;
use rbatis::Page;
//查询商品分类列表
#[pre_authorize("ap:ProductCategory:list")]
pub async fn list(dto: Json<ApProductCategoryPageDTO>) -> impl IntoResponse {
    let data = AP_CONTEXT.ap_product_category_service.page(&dto.0).await;
    let data = data.map(|d| Page::<ApProductCategoryListVO>::from(d));
    PageVO::from_result(&data).into_response()
}
//获取商品分类详细信息
#[pre_authorize("ap:ProductCategory:query")]
pub async fn detail(id: Path<String>) -> impl IntoResponse {
    let product_category = AP_CONTEXT.ap_product_category_service.detail(&id.0).await;
    let product_category = product_category.map(|d| ApProductCategoryVO::from(d));
    RespVO::from_result(&product_category).into_response()
}
//新增商品分类
#[pre_authorize("ap:ProductCategory:add", user_cache)]
pub async fn add(dto: crate::ValidatedForm<ApProductCategoryAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user_cache, ApProductCategory);
    let res = AP_CONTEXT.ap_product_category_service.add(data).await;
    RespVO::from_result(&res).into_response()
}
//更新商品分类
#[pre_authorize("ap:ProductCategory:edit", user_cache)]
pub async fn update(dto: crate::ValidatedForm<ApProductCategoryUpdateDTO>) -> impl IntoResponse {
    update_marco!(data, dto, user_cache, ApProductCategory);
    let res = AP_CONTEXT.ap_product_category_service.update(data).await;
    RespVO::from_result(&res).into_response()
}
//删除商品分类
#[pre_authorize("ap:ProductCategory:remove")]
pub async fn remove(id: Path<String>) -> impl IntoResponse {
    let rows_affected = AP_CONTEXT.ap_product_category_service.remove_batch(&id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "删除成功！", "删除失败！").into_response()
}
//导出商品分类
export_excel_controller!(
    "ap:ProductCategory:export",
    ApProductCategoryPageDTO,
    AP_CONTEXT,
    ap_product_category_service,
    export_as_excel_bytes
);
