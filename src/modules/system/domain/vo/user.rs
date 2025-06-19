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
    #[excel("用户序号", width = 20.0)]
    pub user_id: Option<String>,
    #[excel("部门编号")]
    pub dept_id: Option<String>,
    #[excel("登录名称")]
    pub user_name: Option<String>,
    #[excel("用户昵称")]
    pub nick_name: Option<String>,
    #[excel("用户邮箱")]
    pub email: Option<String>,
    #[excel("手机号码")]
    pub phonenumber: Option<String>,
    #[excel("用户性别", readConverterExp = "0=男,1=女,2=未知")]
    pub sex: Option<char>,
    pub avatar: Option<String>,
    pub password: Option<String>,
    #[excel("帐号状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    pub del_flag: Option<char>,
    #[excel("最后登录IP")]
    pub login_ip: Option<String>,
    #[excel("最后登录时间")]
    #[serde(with = "crate::utils::date_time_format")]
    pub login_date: Option<DateTime>,
    pub create_by: Option<String>,
    #[serde(with = "crate::utils::date_time_format")]
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    pub admin: bool,
    //fixme 将移除，以提高性能
    pub dept: Option<SysDeptVO>,
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
