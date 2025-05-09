use rbatis::crud;
crud!(SysRoleDept {});

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRoleDept {
    //角色id
    pub role_id: Option<String>,
    //菜单id
    pub dept_id: Option<String>,
}