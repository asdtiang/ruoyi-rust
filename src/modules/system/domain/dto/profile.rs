use crate::system::domain::mapper::sys_user::SysUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasswordUpdateDTO {
    pub old_password: Option<String>,
    pub new_password: Option<String>
}

#[derive(Serialize, Deserialize,validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileUpdateDTO {
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
    
}

impl From<ProfileUpdateDTO> for SysUser {
    fn from(arg: ProfileUpdateDTO) -> Self {
        SysUser {
            user_id: None,
            dept_id: None,
            user_name: None,
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: None,
            status: None,
            del_flag: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: None,
            dept: None,
        }
    }
}