use crate::ap::domain::mapper::ap_product_unit::ApProductUnit;
//详情
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProductUnitVO {
    //
    pub id: Option<u64>,
    // 名称
    pub name: Option<String>,
    // 备注
    pub remark: Option<String>,
}
impl From<ApProductUnit> for ApProductUnitVO {
    fn from(arg: ApProductUnit) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            remark: arg.remark,
        }
    }
}
//查询列表用的
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct ApProductUnitListVO {
    //
    pub id: Option<u64>,
    //
}
impl From<ApProductUnit> for ApProductUnitListVO {
    fn from(arg: ApProductUnit) -> Self {
        Self { id: arg.id }
    }
}
