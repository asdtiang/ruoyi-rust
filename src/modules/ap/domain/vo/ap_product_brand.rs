use crate::ap::domain::mapper::ap_product_brand::ApProductBrand;
//详情
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProductBrandVO {
    //
    pub id: Option<u64>,
    // 名称
    pub name: Option<String>,
    // logo
    pub logo: Option<String>,
    // 创建者ID
    pub create_id: Option<u64>,
    // 更新者ID
    pub update_id: Option<u64>,
    // 备注
    pub remark: Option<String>,
}
impl From<ApProductBrand> for ApProductBrandVO {
    fn from(arg: ApProductBrand) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            logo: arg.logo,
            create_id: arg.create_id,
            update_id: arg.update_id,
            remark: arg.remark,
        }
    }
}
//查询列表用的
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct ApProductBrandListVO {
    //
    pub id: Option<u64>,
    pub name: Option<String>,
    // logo
    pub logo: Option<String>,
    // 创建者ID
    #[excel("创建者ID")]
    pub create_id: Option<u64>,
    // 更新者ID
    #[excel("更新者ID")]
    pub update_id: Option<u64>,
    //
}
impl From<ApProductBrand> for ApProductBrandListVO {
    fn from(arg: ApProductBrand) -> Self {
        Self {
            id: arg.id,
            name:arg.name,
            logo:arg.logo,
            create_id: arg.create_id,
            update_id: arg.update_id,
        }
    }
}
