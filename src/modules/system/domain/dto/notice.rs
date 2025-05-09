use  crate::system::domain::mapper::sys_notice::SysNotice;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticePageDTO {
    pub notice_title: Option<String>,
    pub create_by: Option<String>,
    pub notice_type: Option<char>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeAddDTO {
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<char>,
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
            create_time: DateTime::now().set_nano(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoticeUpdateDTO {
    pub notice_id: Option<String>,
    pub notice_title: Option<String>,
    pub notice_content: Option<String>,
    pub notice_type: Option<char>,
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
            update_time: DateTime::now().set_nano(0).into(),
            remark: arg.remark,
        }
    }
}
