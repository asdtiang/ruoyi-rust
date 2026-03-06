use crate::ap::domain::mapper::ap_ap_supplier::ApSupplier;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};
#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApSupplierPageDTO {
    // 供应商名称
    pub name: Option<String>,
    // 联系微信
    pub contact_wechat: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 地址
    pub address: Option<String>,
}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApSupplierAddDTO {
    // 供应商名称
    #[validate(custom(function = "crate::string_required", message = "供应商名称不能为空"))]
    #[validate(length(max = 128, message = "供应商名称不能超过128个字符"))]
    pub name: Option<String>,
    // 系统编码
    #[validate(custom(function = "crate::string_required", message = "系统编码不能为空"))]
    #[validate(length(max = 255, message = "系统编码不能超过255个字符"))]
    pub system_code: Option<String>,
    // 简称
    #[validate(custom(function = "crate::string_required", message = "简称不能为空"))]
    #[validate(length(max = 255, message = "简称不能超过255个字符"))]
    pub short_name: Option<String>,
    // 联系人
    #[validate(length(max = 255, message = "联系人不能超过255个字符"))]
    pub contact_man: Option<String>,
    // 联系方式
    #[validate(length(max = 255, message = "联系方式不能超过255个字符"))]
    pub contact_phone: Option<String>,
    // 联系信息
    #[validate(length(max = 1000, message = "联系信息不能超过1000个字符"))]
    pub contact_info: Option<String>,
    // 联系微信
    #[validate(length(max = 50, message = "联系微信不能超过50个字符"))]
    pub contact_wechat: Option<String>,
    // 链接一
    #[validate(length(max = 255, message = "链接一不能超过255个字符"))]
    pub url_one: Option<String>,
    // 链接二
    #[validate(length(max = 255, message = "链接二不能超过255个字符"))]
    pub url_two: Option<String>,
    // 邮箱
    #[validate(length(max = 50, message = "邮箱不能超过50个字符"))]
    pub email: Option<String>,
    // 地址
    #[validate(length(max = 50, message = "地址不能超过50个字符"))]
    pub address: Option<String>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApSupplierAddDTO> for ApSupplier {
    fn from(arg: ApSupplierAddDTO) -> Self {
        ApSupplier {
            id: None,
            name: arg.name,
            system_code: arg.system_code,
            short_name: arg.short_name,
            contact_man: arg.contact_man,
            contact_phone: arg.contact_phone,
            contact_info: arg.contact_info,
            contact_wechat: arg.contact_wechat,
            url_one: arg.url_one,
            url_two: arg.url_two,
            email: arg.email,
            address: arg.address,
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
pub struct ApSupplierUpdateDTO {
    pub id: Option<u64>,
    // 供应商名称
    #[validate(custom(function = "crate::string_required", message = "供应商名称不能为空"))]
    #[validate(length(max = 128, message = "供应商名称不能超过128个字符"))]
    pub name: Option<String>,
    // 系统编码
    #[validate(custom(function = "crate::string_required", message = "系统编码不能为空"))]
    #[validate(length(max = 255, message = "系统编码不能超过255个字符"))]
    pub system_code: Option<String>,
    // 简称
    #[validate(custom(function = "crate::string_required", message = "简称不能为空"))]
    #[validate(length(max = 255, message = "简称不能超过255个字符"))]
    pub short_name: Option<String>,
    // 联系人
    #[validate(length(max = 255, message = "联系人不能超过255个字符"))]
    pub contact_man: Option<String>,
    // 联系方式
    #[validate(length(max = 255, message = "联系方式不能超过255个字符"))]
    pub contact_phone: Option<String>,
    // 联系信息
    #[validate(length(max = 1000, message = "联系信息不能超过1000个字符"))]
    pub contact_info: Option<String>,
    // 联系微信
    #[validate(length(max = 50, message = "联系微信不能超过50个字符"))]
    pub contact_wechat: Option<String>,
    // 链接一
    #[validate(length(max = 255, message = "链接一不能超过255个字符"))]
    pub url_one: Option<String>,
    // 链接二
    #[validate(length(max = 255, message = "链接二不能超过255个字符"))]
    pub url_two: Option<String>,
    // 邮箱
    #[validate(length(max = 50, message = "邮箱不能超过50个字符"))]
    pub email: Option<String>,
    // 地址
    #[validate(length(max = 50, message = "地址不能超过50个字符"))]
    pub address: Option<String>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApSupplierUpdateDTO> for ApSupplier {
    fn from(arg: ApSupplierUpdateDTO) -> Self {
        ApSupplier {
            id: None,
            name: arg.name,
            system_code: arg.system_code,
            short_name: arg.short_name,
            contact_man: arg.contact_man,
            contact_phone: arg.contact_phone,
            contact_info: arg.contact_info,
            contact_wechat: arg.contact_wechat,
            url_one: arg.url_one,
            url_two: arg.url_two,
            email: arg.email,
            address: arg.address,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
