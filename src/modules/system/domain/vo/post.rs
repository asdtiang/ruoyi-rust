use  crate::system::domain::mapper::sys_post::SysPost;
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive( macros::Export)]
pub struct SysPostVO {
    /** 岗位序号 */
    #[excel( "岗位序号")]
    pub post_id: Option<String>,
    /** 岗位编码 */
    #[excel( "岗位编码")]
    pub post_code: Option<String>,
    /** 岗位名称 */
    #[excel( "岗位名称")]
    pub post_name: Option<String>,
    /** 岗位排序 */
    #[excel( "岗位排序")]
    pub post_sort: Option<u16>,
    /** 状态（0正常 1停用） */
    #[excel( "状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<SysPost> for SysPostVO {
    fn from(arg: SysPost) -> Self {
        Self {
            post_id: arg.post_id,
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark
        }
    }
}


