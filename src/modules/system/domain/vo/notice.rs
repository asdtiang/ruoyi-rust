use crate::system::domain::mapper::sys_notice::SysNotice;
use macros::Export;
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Export)]
pub struct SysNoticeVO {
    /** 公告ID */
    pub notice_id: Option<String>,
    /** 公告标题 */
    #[excel("标题")]
    pub notice_title: Option<String>,
    /** 公告内容 */
    pub notice_content: Option<String>,
    /** 公告类型（1通知 2公告） */
    #[excel("类型", dictType = "sys_notice_type", defaultValue = "12")]
    pub notice_type: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    /** 公告状态（0正常 1关闭） */
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<SysNotice> for SysNoticeVO {
    fn from(arg: SysNotice) -> Self {
        Self {
            notice_id: arg.notice_id,
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            create_by: arg.create_by,
            create_time: arg.create_time,
            status: arg.status,
            remark: arg.remark,
        }
    }
}
