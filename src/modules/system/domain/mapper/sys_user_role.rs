use rbatis::{crud, impl_select};
crud!(SysUserRole {});

///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 角色ID */
    pub role_id: Option<String>,
}
impl_select!(SysUserRole{select_by_user_id_status(user_id: &str,status: char) =>
    " a, sys_role b where a.user_id= #{user_id} and a.role_id= b.role_id and b.status=#{status}"});