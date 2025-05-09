pub mod gen_constants;
pub mod gen_table_column_service;
pub mod gen_table_service;
pub mod gen_utils;
pub mod jinja_utils;

pub use gen_constants::*;
pub use gen_table_column_service::*;
pub use gen_table_service::*;
pub use gen_utils::*;

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
