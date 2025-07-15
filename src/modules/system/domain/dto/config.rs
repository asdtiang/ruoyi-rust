use crate::system::domain::mapper::sys_config::SysConfig;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigPageDTO {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_type: Option<char>,
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigAddDTO {
    /** 参数名称 */
    #[validate(custom(function = "crate::string_required", message = "参数名称不能为空"))]
    #[validate(length(max = 100, message = "参数名称不能超过100个字符"))]
    pub config_name: Option<String>,
    /** 参数键名 */
    #[validate(custom(function = "crate::string_required", message = "参数键名长度不能为空"))]
    #[validate(length(max = 100, message = "参数键名长度不能超过100个字符"))]
    pub config_key: Option<String>,
    /** 参数键值 */
    #[validate(custom(function = "crate::string_required", message = "参数键值不能为空"))]
    #[validate(length(max = 500, message = "参数键值长度不能超过500个字符"))]
    pub config_value: Option<String>,
    /** 系统内置（Y是 N否） */
    pub config_type: Option<char>,
    pub remark: Option<String>,
}

impl From<ConfigAddDTO> for SysConfig {
    fn from(arg: ConfigAddDTO) -> Self {
        SysConfig {
            config_id: ObjectId::new().to_string().into(),
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize,  Clone, Debug)]
#[derive(validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdateDTO {
    /** 参数主键 */
    pub config_id: Option<String>,
    /** 参数名称 */
    #[validate(custom(function = "crate::string_required", message = "参数名称不能为空"))]
    #[validate(length(max = 100, message = "参数名称不能超过100个字符"))]
    pub config_name: Option<String>,
    /** 参数键名 */
    #[validate(custom(function = "crate::string_required", message = "参数键名长度不能为空"))]
    #[validate(length(max = 100, message = "参数键名长度不能超过100个字符"))]
    pub config_key: Option<String>,
    /** 参数键值 */
    #[validate(custom(function = "crate::string_required", message = "参数键值不能为空"))]
    #[validate(length(max = 500, message = "参数键值长度不能超过500个字符"))]
    pub config_value: Option<String>,
    /** 系统内置（Y是 N否） */
    pub config_type: Option<char>,
    pub remark: Option<String>,
}

impl From<ConfigUpdateDTO> for SysConfig {
    fn from(arg: ConfigUpdateDTO) -> Self {
        SysConfig {
            config_id: arg.config_id,
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
