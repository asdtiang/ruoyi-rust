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
    if do_count == false:
     ` order by dict_sort`"});
impl_select!(SysDictData{select_by_dict_type(dict_type:&String)=>"`where dict_type =#{dict_type} order by dict_sort`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictData {
    pub dict_code: Option<String>,
    pub dict_sort: Option<u32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<String>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}