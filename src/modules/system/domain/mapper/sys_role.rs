use crate::system::domain::dto::RolePageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysRole {});
impl_select_page!(SysRole{select_page(dto:&RolePageDTO)=>
    "`where del_flag = '0'`
    if dto.role_id != '':
      ` and role_id = #{dto.role_id}`
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
    /** 角色ID */
    pub role_id: Option<String>,
    /** 角色名称 */
    pub role_name: Option<String>,
    /** 角色权限 */
    pub role_key: Option<String>,
    /** 角色排序 */
    pub role_sort: Option<u32>,
    /** 数据范围（1：所有数据权限；2：自定义数据权限；3：本部门数据权限；4：本部门及以下数据权限；5：仅本人数据权限） */
    pub data_scope: Option<char>,
    /** 菜单树选择项是否关联显示（ 0：父子不互相关联显示 1：父子互相关联显示） */
    pub menu_check_strictly: Option<char>,
    /** 部门树选择项是否关联显示（0：父子不互相关联显示 1：父子互相关联显示 ） */
    pub dept_check_strictly: Option<char>,
    /** 角色状态（0正常 1停用） */
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
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
