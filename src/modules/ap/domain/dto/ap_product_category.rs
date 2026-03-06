use crate::ap::domain::mapper::ap_product_category::ApProductCategory;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};
#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApProductCategoryPageDTO {
    // 上级分类id
    pub parent_id: Option<i32>,
    // 创建者ID
    pub create_id: Option<u64>,
    // 更新者ID
    pub update_id: Option<u64>,
    // 排序
    pub order_num: Option<i32>,
}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApProductCategoryAddDTO {
    // 名称
    #[validate(custom(function = "crate::string_required", message = "名称不能为空"))]
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // 上级分类id
    pub parent_id: Option<i32>,
    // 创建者ID
    #[validate(required(message = "创建者ID不能为空"))]
    pub create_id: Option<u64>,
    // 更新者ID
    #[validate(required(message = "更新者ID不能为空"))]
    pub update_id: Option<u64>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
    // 排序
    pub order_num: Option<i32>,
}
impl From<ApProductCategoryAddDTO> for ApProductCategory {
    fn from(arg: ApProductCategoryAddDTO) -> Self {
        ApProductCategory {
            id: None,
            name: arg.name,
            parent_id: arg.parent_id,
            create_id: arg.create_id,
            create_by: None,
            create_time: None,
            update_id: arg.update_id,
            update_by: None,
            update_time: None,
            remark: arg.remark,
            order_num: arg.order_num,
        }
    }
}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApProductCategoryUpdateDTO {
    pub id: Option<u64>,
    // 名称
    #[validate(custom(function = "crate::string_required", message = "名称不能为空"))]
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // 上级分类id
    pub parent_id: Option<i32>,
    // 创建者ID
    #[validate(required(message = "创建者ID不能为空"))]
    pub create_id: Option<u64>,
    // 更新者ID
    #[validate(required(message = "更新者ID不能为空"))]
    pub update_id: Option<u64>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
    // 排序
    pub order_num: Option<i32>,
}
impl From<ApProductCategoryUpdateDTO> for ApProductCategory {
    fn from(arg: ApProductCategoryUpdateDTO) -> Self {
        ApProductCategory {
            id: None,
            name: arg.name,
            parent_id: arg.parent_id,
            create_id: arg.create_id,
            create_by: None,
            create_time: None,
            update_id: arg.update_id,
            update_by: None,
            update_time: None,
            remark: arg.remark,
            order_num: arg.order_num,
        }
    }
}
