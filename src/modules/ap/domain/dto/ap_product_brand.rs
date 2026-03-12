use crate::ap::domain::mapper::ap_product_brand::ApProductBrand;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};
#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApProductBrandPageDTO {
    // 创建者ID
    pub create_id: Option<u64>,
    // 更新者ID
    pub update_id: Option<u64>,
}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApProductBrandAddDTO {
    // 名称
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // logo
    #[validate(length(max = 255, message = "logo不能超过255个字符"))]
    pub logo: Option<String>,
    // 创建者ID
    pub create_id: Option<u64>,
    // 更新者ID
    pub update_id: Option<u64>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApProductBrandAddDTO> for ApProductBrand {
    fn from(arg: ApProductBrandAddDTO) -> Self {
        ApProductBrand {
            id: None,
            name: arg.name,
            logo: arg.logo,
            create_id: arg.create_id,
            create_by: None,
            create_time: None,
            update_id: arg.update_id,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApProductBrandUpdateDTO {
    pub id: Option<u64>,
    // 名称
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // logo
    #[validate(length(max = 255, message = "logo不能超过255个字符"))]
    pub logo: Option<String>,
    // 创建者ID
    #[validate(required(message = "创建者ID不能为空"))]
    pub create_id: Option<u64>,
    // 创建人头像
    #[validate(length(max = 255, message = "创建人头像不能超过255个字符"))]
    pub create_header_img: Option<String>,
    // 更新者ID
    #[validate(required(message = "更新者ID不能为空"))]
    pub update_id: Option<u64>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApProductBrandUpdateDTO> for ApProductBrand {
    fn from(arg: ApProductBrandUpdateDTO) -> Self {
        ApProductBrand {
            id: None,
            name: arg.name,
            logo: arg.logo,
            create_id: arg.create_id,
            create_by: None,
            create_time: None,
            update_id: arg.update_id,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
