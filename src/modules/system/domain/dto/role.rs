use crate::config::global_constants::DEL_FLAG_NORMAL;
use  crate::system::domain::mapper::sys_role::SysRole;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[page_request(params,dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RolePageDTO {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub status: Option<char>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleAddDTO {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub menu_ids: Option<Vec<String>>,
    pub status: Option<char>,
    pub remark: Option<String>,
}




impl From<RoleAddDTO> for SysRole {
    fn from(arg: RoleAddDTO) -> Self {
        SysRole {
            role_id: ObjectId::new().to_string().into(),
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: match arg.menu_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            dept_check_strictly:  match arg.dept_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            status: arg.status,
            del_flag: Some(DEL_FLAG_NORMAL),
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
pub struct RoleUpdateDTO {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<char>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub menu_ids: Option<Vec<String>>,
    pub status: Option<char>,
    pub remark: Option<String>,

    pub dept_ids: Option<Vec<String>>
}


impl From<RoleUpdateDTO> for SysRole {
    fn from(arg: RoleUpdateDTO) -> Self {
        SysRole {
            role_id: arg.role_id,
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: match arg.menu_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            dept_check_strictly:  match arg.dept_check_strictly.unwrap_or(true)  {
                true => {Some('1')}
                false => {Some('0')}
            },
            status: arg.status,
            del_flag: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_nano(0).into(),
            remark: arg.remark,
        }
    }
}

//
#[page_request(params,dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleAuthUserPageDTO {
    pub role_id: Option<String>,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>
}

