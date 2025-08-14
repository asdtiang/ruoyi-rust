use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysNotice {});


impl_select_page!(SysNotice{select_page(dto: &crate::system::domain::dto::NoticePageDTO) =>
    "`where 1=1`
    if dto.noticeTitle != '':
      ` and notice_title like #{'%'+dto.noticeTitle+'%'}`
    if dto.createBy != '':
      ` and create_by = #{dto.createBy}`
    if dto.noticeType != '':
      ` and notice_type = #{dto.noticeType}`
    ` order by notice_title`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysNotice {
    /** 公告ID */
    pub notice_id: Option<String>,
    /** 公告标题 */
    pub notice_title: Option<String>,
    /** 公告内容 */
    pub notice_content: Option<String>,
    /** 公告类型（1通知 2公告） */
    pub notice_type: Option<char>,
    /** 公告状态（0正常 1关闭） */
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}