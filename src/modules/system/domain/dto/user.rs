use crate::config::global_constants::{DEL_FLAG_NORMAL, STATUS_NORMAL};
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAddDTO {
    /** 归属部门ID */
    #[validate(custom(function = "crate::string_required", message = "归属部门不能为空"))]
    pub dept_id: Option<String>,
    #[validate(custom(function = "crate::xss_validator", message = "用户账号不能包含脚本字符"))]
    /** 用户账号 */
    #[validate(custom(function = "crate::xss_validator", message = "用户账号不能包含脚本字符"))]
    #[validate(custom(function = "crate::string_required", message = "用户账号不能为空"))]
    #[validate(length(max = 30, message = "用户账号长度不能超过30个字符"))]
    pub user_name: Option<String>,
    #[validate(custom(function = "crate::xss_validator", message = "用户昵称不能包含脚本字符"))]
    /** 用户昵称 */
    #[validate(custom(function = "crate::xss_validator", message = "用户昵称不能包含脚本字符"))]
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nick_name: Option<String>,
    /** 用户邮箱 */
    //todo 当前台传入"",导致引起校验
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 50, message = "邮箱长度不能超过50个字符"))]
    pub email: Option<String>,
    /** 手机号码 */
    #[validate(length(max = 11, message = "手机号码长度不能超过11个字符"))]
    pub phonenumber: Option<String>,
    /** 用户性别 */
    pub sex: Option<char>,
    /** 密码 */
    pub password: Option<String>,
    pub remark: Option<String>,

    /** 角色组 */
    pub role_ids: Option<Vec<String>>,
    /** 岗位组 */
    pub post_ids: Option<Vec<String>>,
}

impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            user_id: ObjectId::new().to_string().into(),
            dept_id: arg.dept_id,
            user_name: arg.user_name,
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: arg.password,
            status: STATUS_NORMAL.into(),
            del_flag: DEL_FLAG_NORMAL.into(),
            last_chn_pwd_time: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
            dept: None,
        }
    }
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDTO {
    /** 用户ID */
    #[validate(custom(function = "crate::string_required", message = "用户id不能为空"))]
    pub user_id: Option<String>,
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 用户昵称 */
    #[validate(custom(function = "crate::xss_validator", message = "用户昵称不能包含脚本字符"))]
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nick_name: Option<String>,
    /** 用户邮箱 */
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 50, message = "邮箱长度不能超过50个字符"))]
    pub email: Option<String>,
    /** 手机号码 */
    #[validate(length(max = 11, message = "手机号码长度不能超过11个字符"))]
    pub phonenumber: Option<String>,
    /** 用户性别 */
    pub sex: Option<char>,
    /** 帐号状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    pub remark: Option<String>,
    /** 密码 */
    pub password: Option<String>,
    /** 角色组 */
    pub role_ids: Option<Vec<String>>,
    /** 岗位组 */
    pub post_ids: Option<Vec<String>>,
}

impl From<UserUpdateDTO> for SysUser {
    fn from(arg: UserUpdateDTO) -> Self {
        SysUser {
            user_id: arg.user_id,
            dept_id: arg.dept_id,
            user_name: None,
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: None,
            status: arg.status,
            del_flag: None,
            last_chn_pwd_time: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
            dept: None,
        }
    }
}
#[page_request(params, dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserPageDTO {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub status: Option<String>,
    pub dept_id: Option<String>,
}

impl From<&UserRolePageDTO> for UserPageDTO {
    fn from(arg: &UserRolePageDTO) -> Self {
        Self {
            page_no: arg.page_no.clone(),
            page_size: arg.page_size.clone(),
            user_id: None,
            user_name: arg.user_name.clone(),
            phonenumber: None,
            status: None,
            dept_id: None,
            params: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleDTO {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 角色ID */
    pub role_id: Option<String>,
}

impl From<UserRoleDTO> for SysUserRole {
    fn from(arg: UserRoleDTO) -> Self {
        SysUserRole {
            user_id: arg.user_id,
            role_id: arg.role_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsersRoleDTO {
    pub user_ids: String,
    /** 角色ID */
    pub role_id: String,
}

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    pub user_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleAuthQueryDTO {
    /** 用户ID */
    pub user_id: Option<String>,
    /** 角色组 */
    pub role_ids: Option<String>,
}
