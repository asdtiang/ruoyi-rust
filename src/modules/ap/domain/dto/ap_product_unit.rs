use crate::ap::domain::mapper::ap_product_unit::ApProductUnit;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};
#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApProductUnitPageDTO {}
#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApProductUnitAddDTO {
    // 名称
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApProductUnitAddDTO> for ApProductUnit {
    fn from(arg: ApProductUnitAddDTO) -> Self {
        ApProductUnit {
            id: None,
            name: arg.name,
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
pub struct ApProductUnitUpdateDTO {
    pub id: Option<u64>,
    // 名称
    #[validate(length(max = 255, message = "名称不能超过255个字符"))]
    pub name: Option<String>,
    // 备注
    #[validate(length(max = 500, message = "备注不能超过500个字符"))]
    pub remark: Option<String>,
}
impl From<ApProductUnitUpdateDTO> for ApProductUnit {
    fn from(arg: ApProductUnitUpdateDTO) -> Self {
        ApProductUnit {
            id: None,
            name: arg.name,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}
