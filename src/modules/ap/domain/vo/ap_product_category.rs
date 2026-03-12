use crate::ap::domain::mapper::ap_product_category::ApProductCategory;
//详情
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProductCategoryVO {
    //
    pub id: Option<u64>,
    // 名称
    pub name: Option<String>,
    // 上级分类id
    pub parent_id: Option<u64>,
    // 上级分类名称
    pub parent_name: Option<String>,
    // 创建者ID
    pub create_id: Option<u64>,
    // 更新者ID
    pub update_id: Option<u64>,
    // 备注
    pub remark: Option<String>,
    // 排序
    pub order_num: Option<i32>,
}
impl From<ApProductCategory> for ApProductCategoryVO {
    fn from(arg: ApProductCategory) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            parent_name: arg.parent_name,
            create_id: arg.create_id,
            update_id: arg.update_id,
            remark: arg.remark,
            order_num: arg.order_num,
        }
    }
}
//查询列表用的
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct ApProductCategoryListVO {
    //
    pub id: Option<u64>,
    // 名称
    #[excel("名称")]
    pub name: Option<String>,
    // 上级分类id
    #[excel("上级分类id")]
    pub parent_id: Option<u64>,
    // 上级分类名称
    #[excel("上级分类名称")]
    pub parent_name: Option<String>,
    // 创建者ID
    #[excel("创建者ID")]
    pub create_id: Option<u64>,
    // 更新者ID
    #[excel("更新者ID")]
    pub update_id: Option<u64>,
    // 排序
    #[excel("排序")]
    pub order_num: Option<i32>,
    //
}
impl From<ApProductCategory> for ApProductCategoryListVO {
    fn from(arg: ApProductCategory) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            parent_name: arg.parent_name,
            create_id: arg.create_id,
            update_id: arg.update_id,
            order_num: arg.order_num,
        }
    }
}
