use macros::page_request;
use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperLogPageDTO {
    /** 操作地址 */
    pub oper_ip: Option<String>,
    /** 系统模块*/
    pub title: Option<String>,
    /** 操作人员 */
    pub oper_name: Option<String>,
    /** 业务类型 */
    pub business_type: Option<String>,
    /** 操作状态（0正常 1异常） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>
}
