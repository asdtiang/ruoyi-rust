use crate::system::domain::mapper::sys_notice::SysNotice;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticePageDTO {
    pub notice_title: Option<String>,
    pub create_by: Option<String>,
    pub notice_type: Option<char>,
}


#[derive(Serialize, Deserialize,validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeAddDTO {
    /** 公告标题 */
    #[validate(custom(function = "crate::xss_validator", message = "公告标题不能包含脚本字符"))]
    #[validate(custom(function = "crate::string_required", message = "公告标题不能为空"))]
    #[validate(length(max = 50, message = "公告标题不能超过50个字符"))]
    pub notice_title: Option<String>,
    /** 公告内容 */
    pub notice_content: Option<String>,
    /** 公告类型（1通知 2公告） */
    pub notice_type: Option<char>,
    /** 公告状态（0正常 1关闭） */
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<NoticeAddDTO> for SysNotice {
    fn from(arg: NoticeAddDTO) -> Self {
        SysNotice {
            notice_id: ObjectId::new().to_string().into(),
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize,validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeUpdateDTO {
    /** 公告ID */
    pub notice_id: Option<String>,
    /** 公告标题 */
     #[validate(custom(function = "crate::xss_validator", message = "公告标题不能包含脚本字符"))]
    #[validate(custom(function = "crate::string_required", message = "公告标题不能为空"))]
    #[validate(length(max = 50, message = "公告标题不能超过50个字符"))]
    pub notice_title: Option<String>,
    /** 公告内容 */
    pub notice_content: Option<String>,
    /** 公告类型（1通知 2公告） */
    pub notice_type: Option<char>,
    /** 公告状态（0正常 1关闭） */
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<NoticeUpdateDTO> for SysNotice {
    fn from(arg: NoticeUpdateDTO) -> Self {
        SysNotice {
            notice_id: arg.notice_id,
            notice_title: arg.notice_title,
            notice_content: arg.notice_content,
            notice_type: arg.notice_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
