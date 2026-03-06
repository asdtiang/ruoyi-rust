use crate::ap::domain::mapper::ap_ap_supplier::ApSupplier;
//详情
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApSupplierVO {
    // id
    pub id: Option<u64>,
    // 供应商名称
    pub name: Option<String>,
    // 系统编码
    pub system_code: Option<String>,
    // 简称
    pub short_name: Option<String>,
    // 联系人
    pub contact_man: Option<String>,
    // 联系方式
    pub contact_phone: Option<String>,
    // 联系信息
    pub contact_info: Option<String>,
    // 联系微信
    pub contact_wechat: Option<String>,
    // 链接一
    pub url_one: Option<String>,
    // 链接二
    pub url_two: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 地址
    pub address: Option<String>,
    // 备注
    pub remark: Option<String>,
}
impl From<ApSupplier> for ApSupplierVO {
    fn from(arg: ApSupplier) -> Self {
        Self {
            id: arg.id,
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
            remark: arg.remark,
        }
    }
}
//查询列表用的
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct ApSupplierListVO {
    // id
    pub id: Option<u64>,
    // 供应商名称
    #[excel("供应商名称")]
    pub name: Option<String>,
    // 联系微信
    #[excel("联系微信")]
    pub contact_wechat: Option<String>,
    // 邮箱
    #[excel("邮箱")]
    pub email: Option<String>,
    // 地址
    #[excel("地址")]
    pub address: Option<String>,
    //
}
impl From<ApSupplier> for ApSupplierListVO {
    fn from(arg: ApSupplier) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            contact_wechat: arg.contact_wechat,
            email: arg.email,
            address: arg.address,
        }
    }
}
