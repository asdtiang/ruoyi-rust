use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select, impl_select_page};

crud!(SysDictData {});


impl_select_page!(SysDictData{select_page(dto: &crate::system::domain::dto::DictDataPageDTO) =>
    "`where 1=1 `
    if dto.dictType != '':
      ` and dict_type = #{dto.dictType}`
    if dto.dictLabel != '':
      ` and dict_label like #{'%'+dto.dictLabel+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    ` order by dict_sort`"});
impl_select!(SysDictData{select_by_dict_type(dict_type:&str)=>"`where dict_type =#{dict_type} order by dict_sort`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictData {
    /** 字典编码 */
    pub dict_code: Option<String>,
    /** 字典排序 */
    pub dict_sort: Option<u32>,
    /** 字典标签 */
    pub dict_label: Option<String>,
    /** 字典键值 */
    pub dict_value: Option<String>,
    /** 字典类型 */
    pub dict_type: Option<String>,
    /** 样式属性（其他样式扩展） */
    pub css_class: Option<String>,
    /** 表格字典样式 */
    pub list_class: Option<String>,
    /** 是否默认（Y是 N否） */
    pub is_default: Option<String>,
    /** 状态（0正常 1停用） */
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}