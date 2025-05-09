use  crate::system::domain::mapper::sys_oper_log::SysOperLog;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogVO {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub business_type: Option<u16>,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: Option<u16>,
    pub oper_name: Option<String>,
    pub dept_name: Option<String>,
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<char>,
    pub error_msg: Option<String>,
    pub oper_time: Option<DateTime>,
    pub cost_time: Option<u64>,
}

impl From<SysOperLog> for SysOperLogVO {
    fn from(arg: SysOperLog) -> Self {
        let SysOperLog {
            oper_id,
            title,
            business_type,
            method,
            request_method,
            operator_type,
            oper_name,
            dept_name,
            oper_url,
            oper_ip,
            oper_location,
            oper_param,
            json_result,
            status,
            error_msg,
            oper_time,
            cost_time,
        } = arg;
        Self {
            oper_id,
            title,
            business_type,
            method,
            request_method,
            operator_type,
            oper_name,
            dept_name,
            oper_url,
            oper_ip,
            oper_location,
            oper_param,
            json_result,
            status,
            error_msg,
            oper_time,
            cost_time,
        }
    }
}
