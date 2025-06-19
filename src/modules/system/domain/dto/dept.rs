use crate::config::global_constants::{DEL_FLAG_NORMAL, STATUS_NORMAL};
use  crate::system::domain::mapper::sys_dept::SysDept;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/// dept query DTO
#[page_request(noPage,params,dataScope)]
#[derive(Serialize, Deserialize, Clone, Debug,Default)]
#[serde(rename_all = "camelCase")]
pub struct DeptQueryDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub dept_name: Option<String>,
    pub status: Option<char>,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptAddDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
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
            create_time: DateTime::now().set_nano(0).into(),
            update_by: None,
            update_time: None,

        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptUpdateDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<char>,
    pub del_flag: Option<char>,
}

impl From<DeptUpdateDTO> for SysDept {
    fn from(arg: DeptUpdateDTO) -> Self {
        SysDept {
            dept_id:arg.dept_id,
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
            update_time: DateTime::now().set_nano(0).into(),
        }
    }
}
