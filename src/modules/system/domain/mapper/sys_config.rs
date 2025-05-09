use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysConfig {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
crud!(SysConfig {});


impl_select_page!(SysConfig{select_page(dto: &crate::system::domain::dto::ConfigPageDTO) =>
    "`where 1=1`
    if dto.configName != '':
      ` and config_name like #{'%'+dto.configName+'%'}`
    if dto.configKey != '':
      ` and config_key like #{'%'+dto.configKey+'%'}`
    if dto.configType != '':
      ` and config_type = #{dto.configType}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if dto.params.beginTime != '':
      ` and date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
      ` and date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    if do_count == false:
     ` order by create_time`"});

