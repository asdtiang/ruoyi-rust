use super::super::dto::ApSupplierPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, pysql_select_page};
crud!(ApSupplier {});
impl ApSupplier {
    pysql_select_page!(select_page(dto:&ApSupplierPageDTO) -> ApSupplier =>
    r#"
`SELECT `
if do_count == false:
  ` id, name, contact_wechat, email, address`
if do_count:
  ` count(1)`
` FROM`
` ap_supplier `
` WHERE 1=1`
if dto.id != null:
  ` AND id = #{dto.id}`
if dto.name != '':
  ` AND name like concat('%', #{dto.name}, '%')`
if dto.system_code != '':
  ` AND system_code like concat('%', #{dto.system_code}, '%')`
if dto.contact_phone != '':
  ` AND contact_phone like concat('%', #{dto.contact_phone}, '%')`
if dto.contact_wechat != '':
  ` AND contact_wechat = #{dto.contact_wechat}`
if do_count == false:
  ` ORDER BY create_time desc`
  ` LIMIT ${page_no},${page_size}`
"#);
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApSupplier {
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
    // 创建者
    pub create_by: Option<String>,
    // 创建时间
    pub create_time: Option<DateTime>,
    // 更新者
    pub update_by: Option<String>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 备注
    pub remark: Option<String>,
}
