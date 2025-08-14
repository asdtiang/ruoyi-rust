use crate::system::domain::dto::{RoleAuthUserPageDTO, UserPageDTO};
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select, pysql_select_page};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 用户账号 */
    pub user_name: Option<String>,
    /** 用户昵称 */
    pub nick_name: Option<String>,
    // pub user_type: Option<String>, fixme 目前没有用上
    /** 用户邮箱 */
    pub email: Option<String>,
    /** 手机号码 */
    pub phonenumber: Option<String>,
    /** 用户性别 */
    pub sex: Option<char>,
    /** 用户头像 */
    pub avatar: Option<String>,
    /** 密码 */
    pub password: Option<String>,
    /** 帐号状态（0正常 1停用） */
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
    pub del_flag: Option<char>,
    /** 最后登录IP */
    pub login_ip: Option<String>,
    /** 最后登录时间 */
    pub login_date: Option<DateTime>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}


crud!(SysUser {});

pysql_select_page!(select_page(dto:&UserPageDTO) -> SysUser =>
    r#"`select u.* from sys_user u left join sys_dept d on u.dept_id = d.dept_id where u.del_flag = '0'`
    if dto.userId != '':
      ` and user_id = #{dto.userId}`
    if dto.userName != '':
      ` and user_name like #{'%'+dto.userName+'%'}`
    if dto.phonenumber != '':
      ` and phonenumber like #{'%'+dto.phonenumber+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if dto.params.beginTime != '':
      ` and date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
      ` and date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    if dto.deptId != '':
      ` and (u.dept_id = #{dto.deptId} OR u.dept_id IN ( SELECT t.dept_id FROM sys_dept t WHERE find_in_set(#{dto.deptId}, ancestors)))`
    if dto.params.dataScope != '':
      `${dto.params.dataScope}`
    ` order by u.create_time`
     "#);
impl_select!(SysUser{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});


pysql_select_page!(allocated_user_list(dto: &RoleAuthUserPageDTO) -> SysUser =>
    r#"`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
    ` from sys_user u`
    ` left join sys_dept d on u.dept_id = d.dept_id`
    ` left join sys_user_role ur on u.user_id = ur.user_id`
    ` left join sys_role r on r.role_id = ur.role_id`
    ` where u.del_flag = '0' and r.role_id = #{dto.roleId}`
  if dto.userName != '':
    ` AND u.user_name like concat('%', #{dto.userName}, '%')`
  if dto.phonenumber != '':
    ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
  if dto.params.dataScope != '':
    `${dto.params.dataScope}`
"#);


pysql_select_page!(unallocated_user_list(dto: &RoleAuthUserPageDTO) -> SysUser =>
    r#"`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
 ` from sys_user u`
 ` left join sys_dept d on u.dept_id = d.dept_id`
 ` left join sys_user_role ur on u.user_id = ur.user_id`
 ` left join sys_role r on r.role_id = ur.role_id`
 ` where u.del_flag = '0' and (r.role_id != #{dto.roleId} or r.role_id IS NULL)`
 ` and u.user_id not in (select u.user_id from sys_user u inner join sys_user_role ur on u.user_id = ur.user_id and ur.role_id = #{dto.roleId})`
  if dto.userName != '':
    ` AND u.user_name like concat('%', #{dto.userName}, '%')`
  if dto.phonenumber != '':
    ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
  if dto.params.dataScope != '':
  ` ${dto.params.dataScope}``
"#);

