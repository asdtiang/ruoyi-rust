use rbatis::crud;
crud!(SysRoleDept {});

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRoleDept {
    /** 角色ID */
    pub role_id: Option<String>,
    /** 部门ID */
    pub dept_id: Option<String>,
}