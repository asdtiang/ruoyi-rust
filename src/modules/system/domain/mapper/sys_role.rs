use  crate::system::domain::dto::RolePageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysRole {});
impl_select_page!(SysRole{select_page(dto:&RolePageDTO)=>
    "`where del_flag = '0'`
    if dto.role_id != '':
      ` and role_id = #{dto.role_i}`
    if dto.role_name != '':
      ` and role_name like #{'%'+dto.role_name+'%'}`
    if dto.role_name != '':
      ` and role_key like #{'%'+dto.role_name+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if dto.params.beginTime != '':
      ` and date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
      ` and date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    if dto.params.dataScope != '':
      `${dto.params.dataScope}`
    if do_count == false:
     ` order by role_sort`"
});

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRole {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<char>,
    pub dept_check_strictly: Option<char>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
impl SysRole {
    pub fn is_admin(&self) -> bool {
        self.role_id.clone().unwrap() == "1"
    }
}
