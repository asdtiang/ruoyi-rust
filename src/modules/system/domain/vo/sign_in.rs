use crate::system::domain::mapper::sys_role::SysRole;
use crate::system::domain::vo::user::SysUserVO;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

///的后的所有信息，保存到redis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCache {
    pub id: String,
    pub user_name: String,
    pub user: Option<SysUserVO>,
    pub permissions: Vec<String>,
    pub menu_ids: Vec<String>,
    pub roles: Vec<SysRole>,
    pub login_time: DateTime,
    pub token_key:String
}

impl ToString for UserCache {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
