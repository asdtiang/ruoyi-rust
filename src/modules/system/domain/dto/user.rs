use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};
use  crate::system::domain::mapper::sys_user::SysUser;
use  crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::utils::password_encoder::PasswordEncoder;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate,  Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAddDTO {
    pub dept_id: Option<String>,
   // @Xss(message = "用户账号不能包含脚本字符")
    #[validate(required(message = "用户账号不能为空"))]
    #[validate(length(max = 30,message="用户账号长度不能超过30个字符"))]
    pub user_name: Option<String>,
   // @Xss(message = "用户昵称不能包含脚本字符")
    #[validate(length(max = 30,message="用户昵称长度不能超过30个字符"))]
    pub nick_name: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 50,message="邮箱长度不能超过50个字符"))]
    pub email: Option<String>,
    #[validate(length(max = 11,message="手机号码长度不能超过11个字符"))]
    pub phonenumber: Option<String>,
    pub sex: Option<char>,
    pub password: Option<String>,
    pub remark: Option<String>,

    pub role_ids: Option<Vec<String>>,
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
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            status: STATUS_NORMAL.into(),
            del_flag: DEL_FLAG_NORMAL.into(),
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: DateTime::now().set_nano(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDTO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<char>,
    pub status: Option<char>,
    pub remark: Option<String>,
    pub password: Option<String>,
    pub role_ids: Option<Vec<String>>,
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
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_nano(0).into(),
            remark: arg.remark,
        }
    }
}
#[page_request(params,dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug,Default)]
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
    pub user_id: Option<String>,
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
    pub user_id: Option<String>,
    pub role_ids: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasswordUpdateDTO {
    pub old_password: Option<String>,
    pub new_password: Option<String>
}
