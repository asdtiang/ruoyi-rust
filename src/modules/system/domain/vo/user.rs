use crate::config::global_constants::ADMIN_NAME;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::vo::{SysDeptVO, SysRoleVO};
use macros::Export;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Export)]
pub struct SysUserVO {
    /** 用户ID */
    #[excel( "用户序号")]
    pub user_id: Option<String>,
    /** 部门ID */
    #[excel( "部门编号", attrType = crate::AttrType::IMPORT )]
    pub dept_id: Option<String>,
    /** 用户账号 */
    #[excel( "登录名称")]
    pub user_name: Option<String>,
    /** 用户昵称 */
    #[excel( "用户名称")]
    pub nick_name: Option<String>,
    /** 用户邮箱 */
    #[excel( "用户邮箱")]
    pub email: Option<String>,
    /** 手机号码 */
    #[excel( "手机号码")]
    pub phonenumber: Option<String>,
    /** 用户性别 */
    #[excel( "用户性别", readConverterExp = "0=男,1=女,2=未知")]
    pub sex: Option<char>,
    /** 用户头像 */
    pub avatar: Option<String>,
    /** 密码 */
    pub password: Option<String>,
    /** 帐号状态（0正常 1停用） */
    #[excel( "帐号状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
    pub del_flag: Option<char>,
    /** 最后登录IP */
    #[excel( "最后登录IP", attrType = crate::AttrType::EXPORT )]
    pub login_ip: Option<String>,
    #[serde(with = "crate::utils::date_time_format")]
    #[excel( "最后登录时间", width = 30.0, attrType = crate::AttrType::EXPORT )]
    /** 最后登录时间 */
    pub login_date: Option<DateTime>,
    pub create_by: Option<String>,
    #[serde(with = "crate::utils::date_time_format")]
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    pub admin: bool,
    //fixme 将移除，以提高性能
    /** 部门对象 */
    pub dept: Option<SysDeptVO>,
    /** 角色对象 */
    pub roles: Option<Vec<SysRoleVO>>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            user_id: arg.user_id,
            dept_id: arg.dept_id,
            user_name: arg.user_name.clone(),
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: arg.avatar,
            //屏蔽密码
            password: None,
            status: arg.status,
            del_flag: arg.del_flag,
            login_ip: arg.login_ip,
            login_date: arg.login_date,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
            admin: arg.user_name.unwrap_or_default().eq(ADMIN_NAME),
            dept: None,
            roles: None,
        }
    }
}
