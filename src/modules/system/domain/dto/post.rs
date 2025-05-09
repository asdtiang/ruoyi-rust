use  crate::system::domain::mapper::sys_post::SysPost;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostPageDTO {
    pub post_name: Option<String>,
    pub post_code: Option<String>,
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAddDTO {
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<PostAddDTO> for SysPost {
    fn from(arg: PostAddDTO) -> Self {
        SysPost {
            post_id: ObjectId::new().to_string().into(),
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
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
pub struct PostUpdateDTO {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub post_sort: Option<u16>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<PostUpdateDTO> for SysPost {
    fn from(arg: PostUpdateDTO) -> Self {
        SysPost {
            post_id: arg.post_id,
            post_code: arg.post_code,
            post_name: arg.post_name,
            post_sort: arg.post_sort,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_nano(0).into(),
            remark: arg.remark,
        }
    }
}
