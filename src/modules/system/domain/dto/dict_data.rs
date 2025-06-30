use crate::system::domain::mapper::sys_dict_data::SysDictData;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataPageDTO {
    pub dict_type: Option<String>,
    pub dict_label: Option<String>,
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataAddDTO {
    /** 字典排序 */
    pub dict_sort: Option<u32>,
    /** 字典标签 */
    #[validate(custom(function = "crate::string_required", message = "字典标签不能为空"))]
    #[validate(length(max = 100, message = "字典标签长度不能超过100个字符"))]
    pub dict_label: Option<String>,
    /** 字典键值 */
    #[validate(custom(function = "crate::string_required", message = "字典键值不能为空"))]
    #[validate(length(max = 100, message = "字典键值长度不能超过100个字符"))]
    pub dict_value: Option<String>,
    /** 字典类型 */
    #[validate(custom(function = "crate::string_required", message = "字典类型不能为空"))]
    #[validate(length(max = 100, message = "字典类型长度不能超过100个字符"))]
    pub dict_type: Option<String>,
    /** 样式属性（其他样式扩展） */
    #[validate(length(max = 100, message = "样式属性长度不能超过100个字符"))]
    pub css_class: Option<String>,
    /** 表格字典样式 */
    pub list_class: Option<String>,
    /** 是否默认（Y是 N否） */
    pub is_default: Option<String>,
    /** 状态（0正常 1停用） */
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub remark: Option<String>,
}
impl From<DictDataAddDTO> for SysDictData {
    fn from(arg: DictDataAddDTO) -> Self {
        SysDictData {
            dict_code: ObjectId::new().to_string().into(),
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictDataUpdateDTO {
    /** 字典编码 */
    pub dict_code: Option<String>,
    /** 字典排序 */
    pub dict_sort: Option<u32>,
    /** 字典标签 */
    #[validate(custom(function = "crate::string_required", message = "字典标签不能为空"))]
    #[validate(length(max = 100, message = "字典标签长度不能超过100个字符"))]
    pub dict_label: Option<String>,
    /** 字典键值 */
    #[validate(custom(function = "crate::string_required", message = "字典键值不能为空"))]
    #[validate(length(max = 100, message = "字典键值长度不能超过100个字符"))]
    pub dict_value: Option<String>,
    /** 字典类型 */
    #[validate(custom(function = "crate::string_required", message = "字典类型不能为空"))]
    #[validate(length(max = 100, message = "字典类型长度不能超过100个字符"))]
    pub dict_type: Option<String>,
    /** 样式属性（其他样式扩展） */
    #[validate(length(max = 100, message = "样式属性长度不能超过100个字符"))]
    pub css_class: Option<String>,
    /** 表格字典样式 */
    pub list_class: Option<String>,
    /** 是否默认（Y是 N否） */
    pub is_default: Option<String>,
    /** 状态（0正常 1停用） */
    pub status: Option<char>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<DictDataUpdateDTO> for SysDictData {
    fn from(arg: DictDataUpdateDTO) -> Self {
        SysDictData {
            dict_code: arg.dict_code,
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}
