pub mod dict_type;
pub mod dict_data;
pub mod menu;
pub mod role;
pub mod sign_in;
pub mod user;
pub mod config;
pub mod role_menu;
pub mod logininfor;
pub mod dept;
pub mod post;
pub mod notice;
pub mod oper_log;
pub mod profile;

pub use config::*;
pub use dept::*;
pub use dict_data::*;
pub use dict_type::*;
pub use menu::*;
pub use notice::*;
pub use post::*;
pub use role::*;
pub use sign_in::*;
pub use user::*;

pub use logininfor::*;
pub use oper_log::*;
pub use profile::*;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
