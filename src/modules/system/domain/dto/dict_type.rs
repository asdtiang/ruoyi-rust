use crate::system::domain::mapper::sys_dict_type::SysDictType;
use macros::page_request;
use rbatis::object_id::ObjectId;

use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageDTO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeAddDTO {
    /** 字典名称 */
    #[validate(custom(function = "crate::string_required", message = "字典名称不能为空"))]
    #[validate(length(max = 100, message = "字典类型名称长度不能超过100个字符"))]
    pub dict_name: Option<String>,
    /** 字典类型 */
    #[validate(custom(function = "crate::string_required", message = "字典类型不能为空"))]
    #[validate(length(max = 100, message = "字典类型类型长度不能超过100个字符"))]
    #[validate(regex(path =* crate::NORMAL_NAME_REG, message = "字典类型必须以字母开头，且只能为（小写字母，数字，下滑线）"))]
    pub dict_type: Option<String>,
    /** 状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<DictTypeAddDTO> for SysDictType {
    fn from(arg: DictTypeAddDTO) -> Self {
        SysDictType {
            dict_id: ObjectId::new().to_string().into(),
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeUpdateDTO {
    /** 字典主键 */
    pub dict_id: Option<String>,
    /** 字典名称 */
    #[validate(custom(function = "crate::string_required", message = "字典名称不能为空"))]
    #[validate(length(max = 100, message = "字典类型名称长度不能超过100个字符"))]
    pub dict_name: Option<String>,
    /** 字典类型 */
    #[validate(custom(function = "crate::string_required", message = "字典类型不能为空"))]
    #[validate(length(max = 100, message = "字典类型类型长度不能超过100个字符"))]
    #[validate(regex(path =* crate::NORMAL_NAME_REG, message = "字典类型必须以字母开头，且只能为（小写字母，数字，下滑线）"))]
    pub dict_type: Option<String>,
    /** 状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    pub remark: Option<String>,
}
impl From<DictTypeUpdateDTO> for SysDictType {
    fn from(arg: DictTypeUpdateDTO) -> Self {
        SysDictType {
            dict_id: arg.dict_id,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
