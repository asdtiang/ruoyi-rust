use crate::config::global_constants::ADMIN_NAME;
use crate::system::domain::vo::CommonRoleVO;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

///登录后的所有信息，保存到redis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCache {
    pub user_id: String,
    pub user_name: String,
    pub dept_id: String,
    pub dept_name: String,
    pub permissions: Vec<String>,
    pub menu_ids: Vec<String>,
    pub roles: Vec<CommonRoleVO>,
    pub login_time: DateTime,
    pub login_user_key: String,
    pub token_key: String,
    pub need_chn_pwd: bool,
    pub login_ip: String,
    pub browser: String,
    pub os:String,
}

impl UserCache {
    pub fn is_admin(&self) -> bool {
        self.user_name.eq(ADMIN_NAME)
    }
    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }
    pub fn dept_id(&self) -> String {
        self.dept_id.clone()
    }
    pub fn login_user_key(&self) -> String {
        self.login_user_key.clone()
    }
}

impl ToString for UserCache {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}




