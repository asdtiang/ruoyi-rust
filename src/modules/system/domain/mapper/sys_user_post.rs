use rbatis::crud;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserPost {
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub post_id: Option<String>,
}

crud!(SysUserPost {});

