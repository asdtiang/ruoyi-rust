use super::super::dto::ApProductUnitPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, pysql_select_page};
crud!(ApProductUnit {});
impl ApProductUnit {
    pysql_select_page!(select_page(dto:&ApProductUnitPageDTO) -> ApProductUnit =>
    r#"
`SELECT `
if do_count == false:
  ` id`
if do_count:
  ` count(1)`
` FROM`
` ap_product_unit `
` WHERE 1=1`
if dto.name != '':
  ` AND name like concat('%', #{dto.name}, '%')`
if do_count == false:
  ` ORDER BY create_time desc`
  ` LIMIT ${page_no},${page_size}`
"#);
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApProductUnit {
    //
    pub id: Option<u64>,
    // 名称
    pub name: Option<String>,
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
