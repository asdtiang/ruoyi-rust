use crate::system::domain::mapper::sys_post::SysPost;
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

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostAddDTO {
    /** 岗位编码 */
    #[validate(custom(function = "crate::string_required", message = "岗位编码不能为空"))]
    #[validate(length(max = 64, message = "岗位编码长度不能超过64个字符"))]
    pub post_code: Option<String>,
    /** 岗位名称 */
    #[validate(custom(function = "crate::string_required", message = "岗位名称不能为空"))]
    #[validate(length(max = 50, message = "岗位名称长度不能超过50个字符"))]
    pub post_name: Option<String>,
    /** 岗位排序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub post_sort: Option<u16>,
    /** 状态（0正常 1停用） */
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

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostUpdateDTO {
    /** 岗位序号 */
    pub post_id: Option<String>,
    /** 岗位编码 */
    #[validate(custom(function = "crate::string_required", message = "岗位编码不能为空"))]
    #[validate(length(max = 64, message = "岗位编码长度不能超过64个字符"))]
    pub post_code: Option<String>,
    /** 岗位名称 */
    #[validate(custom(function = "crate::string_required", message = "岗位名称不能为空"))]
    #[validate(length(max = 50, message = "岗位名称长度不能超过50个字符"))]
    pub post_name: Option<String>,
    /** 岗位排序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub post_sort: Option<u16>,
    /** 状态（0正常 1停用） */
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
