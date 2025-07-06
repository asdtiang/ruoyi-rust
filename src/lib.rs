pub mod utils;
pub mod config;
pub mod error;
pub mod context;
pub mod web_data;
pub mod token_auth;
pub mod log;
pub mod macros;
pub mod modules;
pub use modules::*;
pub use utils::excel_utils::ExcelGenAttr;
pub use utils::excel_utils::ExcelGenAttrTrait;
pub use utils::excel_utils::AttrType;
pub use utils::validate_utils::string_required;
pub use utils::validator::ValidatedForm;

pub trait DataScopeTrait {
    fn clear_data_scope_params(&mut self) ;
    fn set_data_scope_params(&mut self,value:&str) ;
}
