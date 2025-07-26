pub mod utils;
pub mod config;
pub mod error;
pub mod context;
pub mod log;
pub mod macros;
pub mod modules;
pub mod web;

pub use modules::*;
pub use utils::excel_utils::AttrType;
pub use utils::excel_utils::ExcelGenAttr;
pub use utils::excel_utils::ExcelGenAttrTrait;
pub use utils::validate_utils::*;
pub use web::token::auth::UserCache;
pub use web::extractors::validator::ValidatedForm;

pub trait DataScopeTrait {
    fn clear_data_scope_params(&mut self) ;
    fn set_data_scope_params(&mut self,value:&str) ;
}
