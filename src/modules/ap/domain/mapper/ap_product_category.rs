use super::super::dto::ApProductCategoryPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, pysql_select_page};
crud!(ApProductCategory {});
impl ApProductCategory {
    pysql_select_page!(select_page(dto:&ApProductCategoryPageDTO) -> ApProductCategory =>
    r#"
`SELECT `
if do_count == false:
  ` id, parent_id, create_id, update_id, order_num`
if do_count:
  ` count(1)`
` FROM`
` ap_product_category `
` WHERE 1=1`
if dto.name != '':
  ` AND name like concat('%', #{dto.name}, '%')`
if dto.parent_id != null:
  ` AND parent_id = #{dto.parent_id}`
if do_count == false:
  ` ORDER BY create_time desc`
  ` LIMIT ${page_no},${page_size}`
"#);
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApProductCategory {
    //
    pub id: Option<u64>,
    // 名称
    pub name: Option<String>,
    // 上级分类id
    pub parent_id: Option<i32>,
    // 创建者ID
    pub create_id: Option<u64>,
    // 创建者
    pub create_by: Option<String>,
    // 创建时间
    pub create_time: Option<DateTime>,
    // 更新者ID
    pub update_id: Option<u64>,
    // 更新者
    pub update_by: Option<String>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 备注
    pub remark: Option<String>,
    // 排序
    pub order_num: Option<i32>,
}
