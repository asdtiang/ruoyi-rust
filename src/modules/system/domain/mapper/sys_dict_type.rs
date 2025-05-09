use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysDictType {});

impl_select_page!(SysDictType{select_page(dto: &crate::system::domain::dto::DictTypePageDTO) =>
    "`where 1=1 `
    if dto.dictType != '':
      ` and dict_type like #{'%'+dto.dictType+'%'}`
    if dto.dictName != '':
      ` and dict_name like #{'%'+dto.dictName+'%'}`
    if dto.params.beginTime != '':
      ` and date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
      ` and date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    if dto.status != '':
      ` and status = #{dto.status}`
    if do_count == false:
     ` order by create_time`"});

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictType {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictTypeSimple {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
}