use std::sync::LazyLock;
use axum::Router;
use axum::routing::{delete, get, post, put};
pub mod controller;
pub mod domain;
pub mod service;

pub use controller::*;
use crate::gen::service::{GenTableColumnService, GenTableService};

pub(crate) fn build_gen_api() -> Router {
    Router::new()
        .nest("/gen",gen_table_api())
}
fn gen_table_api() -> Router {
    Router::new()
        .route("/list", post(gen_table_controller::list))
        .route("/db/list", post(gen_table_controller::db_list))
        .route("/", put(gen_table_controller::update))
        .route("/{table_id}", get(gen_table_controller::detail))
        .route("/{table_id}", delete(gen_table_controller::remove))
        .route("/importTable", post(gen_table_controller::import_table))
        .route("/genCode/{table_name}", get(gen_table_controller::batch_gen_code))
        .route("/synchDb/{table_name}", get(gen_table_controller::synch_db))
}

pub static GEN_CONTEXT: LazyLock<GenServiceContext> =
    LazyLock::new(|| GenServiceContext::default());

pub struct GenServiceContext {
    pub gen_table_service: GenTableService,
    pub gen_table_column_service: GenTableColumnService,
    pub config: GenConfig,
}

impl Default for GenServiceContext {
    fn default() -> Self {
        let yml_data = include_str!("../../../generator.yml");
        //load config
        let config: GenConfig = serde_yaml::from_str(yml_data).expect("load config file fail");
        Self {
            gen_table_service: GenTableService {},
            gen_table_column_service: GenTableColumnService {},
            config,
        }
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenConfig {
    /** 作者 */
    pub author: String,

    /** 生成包路径 */
    pub package_name: String,

    /** 自动去除表前缀，默认是false */
    pub auto_remove_pre: bool,

    /** 表前缀(类名不会包含表前缀) */
    pub table_prefixes: Vec<String>,
}