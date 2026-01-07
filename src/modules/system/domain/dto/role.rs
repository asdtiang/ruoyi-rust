use crate::config::global_constants::DEL_FLAG_NORMAL;
use crate::system::domain::mapper::sys_role::SysRole;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};

#[page_request(params,dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RolePageDTO {
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>
}


#[derive(Serialize, Deserialize, validator::Validate,Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleAddDTO {
    /** 角色名称 */
    #[validate(custom(function = "crate::string_required", message = "角色名称不能为空"))]
    #[validate(length(max = 30, message = "角色名称长度不能超过30个字符"))]
    pub role_name: Option<String>,
    /** 角色权限 */
    #[validate(custom(function = "crate::string_required", message = "权限字符不能为空"))]
    #[validate(length(max = 100, message = "权限字符长度不能超过100个字符"))]
    pub role_key: Option<String>,
    /** 角色排序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub role_sort: Option<u32>,
    /** 数据范围（1：所有数据权限；2：自定义数据权限；3：本部门数据权限；4：本部门及以下数据权限；5：仅本人数据权限） */
    pub data_scope: Option<char>,
    /** 菜单树选择项是否关联显示（ 0：父子不互相关联显示 1：父子互相关联显示） */
    pub menu_check_strictly: Option<bool>,
    /** 部门树选择项是否关联显示（0：父子不互相关联显示 1：父子互相关联显示 ） */
    pub dept_check_strictly: Option<bool>,
    /** 菜单组 */
    pub menu_ids: Option<Vec<String>>,
    /** 角色状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
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
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}


#[derive(Serialize, Deserialize, validator::Validate,Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleUpdateDTO {
    /** 角色ID */
    pub role_id: Option<String>,
    /** 角色名称 */
    #[validate(custom(function = "crate::string_required", message = "角色名称不能为空"))]
    #[validate(length(max = 30, message = "角色名称长度不能超过30个字符"))]
    pub role_name: Option<String>,
    /** 角色权限 */
    #[validate(custom(function = "crate::string_required", message = "权限字符不能为空"))]
    #[validate(length(max = 100, message = "权限字符长度不能超过100个字符"))]
    pub role_key: Option<String>,
    /** 角色排序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub role_sort: Option<u32>,
    /** 数据范围（1：所有数据权限；2：自定义数据权限；3：本部门数据权限；4：本部门及以下数据权限；5：仅本人数据权限） */
    pub data_scope: Option<char>,
    /** 菜单树选择项是否关联显示（ 0：父子不互相关联显示 1：父子互相关联显示） */
    pub menu_check_strictly: Option<bool>,
    /** 部门树选择项是否关联显示（0：父子不互相关联显示 1：父子互相关联显示 ） */
    pub dept_check_strictly: Option<bool>,
    /** 菜单组 */
    pub menu_ids: Option<Vec<String>>,
    /** 角色状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    pub remark: Option<String>,

    /** 部门组（数据权限） */
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
            update_time: None,
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

