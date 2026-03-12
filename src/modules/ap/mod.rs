use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::LazyLock;
pub mod controller;
pub mod domain;
pub mod service;
use crate::router_log;
use controller::*;
use service::*;
pub static AP_CONTEXT: LazyLock<ApServiceContext> = LazyLock::new(|| ApServiceContext::default());
pub struct ApServiceContext {
    //gen_service_a
    //autogen_ap_supplier
    pub ap_ap_supplier_service: ApSupplierService,
    //autogen_ap_product_brand
    pub ap_product_brand_service: ApProductBrandService,
    //autogen_ap_product_unit
    pub ap_product_unit_service: ApProductUnitService,
    //autogen_ap_product_category
    pub ap_product_category_service: ApProductCategoryService,
    //endgen_service_a
}
impl Default for ApServiceContext {
    fn default() -> Self {
        Self {
            //gen_service_b
            //autogen_ap_supplier
            ap_ap_supplier_service: ApSupplierService {},
            //autogen_ap_product_brand
            ap_product_brand_service: ApProductBrandService {},
            //autogen_ap_product_unit
            ap_product_unit_service: ApProductUnitService {},
            //autogen_ap_product_category
            ap_product_category_service: ApProductCategoryService {}, //endgen_service_b
        }
    }
}
pub(crate) fn build_ap_api() -> Router {
    Router::new()
        //gen_router_a
        //autogen_ap_supplier
        .nest("/ApSupplier", ap_supplier_api())
        //autogen_ap_product_brand
        .nest("/ProductBrand", product_brand_api())
        //autogen_ap_product_unit
        .nest("/productUnit", product_unit_api())
        //autogen_ap_product_category
        .nest("/ProductCategory", product_category_api())
    //endgen_router_a
}
//gen_router_b
//autogen_ap_supplier
fn ap_supplier_api() -> Router {
    Router::new()
        .route("/list", post(ap_ap_supplier_controller::list))
        .route("/{id}", get(ap_ap_supplier_controller::detail))
        .route("/", router_log!(post, ap_ap_supplier_controller::add, "供应商", "新增"))
        .route(
            "/",
            router_log!(put, ap_ap_supplier_controller::update, "供应商", "修改"),
        )
        .route("/{id}", delete(ap_ap_supplier_controller::remove))
        .route("/export", post(ap_ap_supplier_controller::export_to_excel))
}
//autogen_ap_product_brand
fn product_brand_api() -> Router {
    Router::new()
        .route("/list", post(ap_product_brand_controller::list))
        .route("/{id}", get(ap_product_brand_controller::detail))
        .route(
            "/",
            router_log!(post, ap_product_brand_controller::add, "品牌库", "新增"),
        )
        .route(
            "/",
            router_log!(put, ap_product_brand_controller::update, "品牌库", "修改"),
        )
        .route("/{id}", delete(ap_product_brand_controller::remove))
        .route("/export", post(ap_product_brand_controller::export_to_excel))
}
//autogen_ap_product_unit
fn product_unit_api() -> Router {
    Router::new()
        .route("/list", post(ap_product_unit_controller::list))
        .route("/{id}", get(ap_product_unit_controller::detail))
        .route("/", router_log!(post, ap_product_unit_controller::add, "单位", "新增"))
        .route(
            "/",
            router_log!(put, ap_product_unit_controller::update, "单位", "修改"),
        )
        .route("/{id}", delete(ap_product_unit_controller::remove))
        .route("/export", post(ap_product_unit_controller::export_to_excel))
}
//autogen_ap_product_category
fn product_category_api() -> Router {
    Router::new()
        .route("/list", post(ap_product_category_controller::list))
        .route("/listParent", get(ap_product_category_controller::list_parent))
        .route("/{id}", get(ap_product_category_controller::detail))
        .route(
            "/",
            router_log!(post, ap_product_category_controller::add, "商品分类", "新增"),
        )
        .route(
            "/",
            router_log!(put, ap_product_category_controller::update, "商品分类", "修改"),
        )
        .route("/{id}", delete(ap_product_category_controller::remove))
        .route("/export", post(ap_product_category_controller::export_to_excel))
}
//endgen_router_b
