use crate::config::global_constants::{DEL_FLAG_NORMAL, STATUS_NORMAL};
use crate::system::domain::mapper::sys_dept::SysDept;
use macros::page_request;
use rbatis::object_id::ObjectId;
use serde::{Deserialize, Serialize};

/// dept query DTO
#[page_request(noPage, params, dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeptQueryDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub dept_name: Option<String>,
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptAddDTO {
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 父部门ID */
    pub parent_id: Option<String>,
    /** 祖级列表 */
    pub ancestors: Option<String>,
    /** 部门名称 */
    #[validate(custom(function = "crate::string_required", message = "部门名称不能为空"))]
    #[validate(length(max = 30, message = "部门名称长度不能超过30个字符"))]
    pub dept_name: Option<String>,
    /** 显示顺序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub order_num: Option<u16>,
    /** 负责人 */
    pub leader: Option<String>,
    /** 联系电话 */
    #[validate(length(max = 11, message = "联系电话长度不能超过11个字符"))]
    pub phone: Option<String>,
    /** 邮箱 */
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 50, message = "邮箱长度不能超过50个字符"))]
    pub email: Option<String>,
}

impl From<DeptAddDTO> for SysDept {
    fn from(arg: DeptAddDTO) -> Self {
        SysDept {
            dept_id: ObjectId::new().to_string().into(),
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: Some(STATUS_NORMAL),
            del_flag: Some(DEL_FLAG_NORMAL),
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}

#[derive(Serialize, Deserialize, validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptUpdateDTO {
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 父部门ID */
    pub parent_id: Option<String>,
    /** 祖级列表 */
    pub ancestors: Option<String>,
    /** 部门名称 */
    #[validate(custom(function = "crate::string_required", message = "部门名称不能为空"))]
    #[validate(length(max = 30, message = "部门名称长度不能超过30个字符"))]
    pub dept_name: Option<String>,
    /** 显示顺序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub order_num: Option<u16>,
    /** 负责人 */
    pub leader: Option<String>,
    /** 联系电话 */
    #[validate(length(max = 11, message = "联系电话长度不能超过11个字符"))]
    pub phone: Option<String>,
    /** 邮箱 */
    #[validate(email(message = "邮箱格式不正确"))]
    #[validate(length(max = 50, message = "邮箱长度不能超过50个字符"))]
    pub email: Option<String>,
    /** 部门状态:0正常,1停用 */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
    pub del_flag: Option<char>,
}

impl From<DeptUpdateDTO> for SysDept {
    fn from(arg: DeptUpdateDTO) -> Self {
        SysDept {
            dept_id: arg.dept_id,
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
        }
    }
}
