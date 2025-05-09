use rbatis::crud;
crud!(SysRoleMenu {});

///Role menu relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRoleMenu {
    //角色id
    pub role_id: Option<String>,
    //菜单id
    pub menu_id: Option<String>,
}