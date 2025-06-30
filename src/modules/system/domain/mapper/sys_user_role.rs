use rbatis::crud;
crud!(SysUserRole {});

///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 角色ID */
    pub role_id: Option<String>,
}