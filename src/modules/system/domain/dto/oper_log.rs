use macros::page_request;
use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperLogPageDTO {
    /** 操作地址 */
    pub oper_ip: Option<String>,
    /** 系统模块*/
    pub title: Option<String>,
    /** 操作人员 */
    pub oper_name: Option<String>,
    /** 业务类型（0其它 1新增 2修改 3删除） */
    pub business_type: Option<u16>,
    /** 操作状态（0正常 1异常） */
    pub status: Option<char>
}
