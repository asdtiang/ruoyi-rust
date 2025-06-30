use rbatis::crud;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserPost {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 岗位ID */
    pub post_id: Option<String>,
}

crud!(SysUserPost {});

