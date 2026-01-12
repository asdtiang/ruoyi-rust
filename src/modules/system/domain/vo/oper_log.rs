use crate::system::domain::mapper::sys_oper_log::SysOperLog;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct SysOperLogVO {
    /** 日志主键 */
    #[excel("操作序号")]
    pub oper_id: Option<String>,
    /** 操作模块 */
    #[excel("操作模块")]
    pub title: Option<String>,
    /** 业务类型 */
    #[excel("业务类型" )]
    pub business_type: Option<String>,
    /** 请求方法 */
    #[excel("请求方法")]
    pub method: Option<String>,
    /** 请求方式 */
    #[excel("请求方式")]
    pub request_method: Option<String>,
    /** 操作类别（0其它 1后台用户 2手机端用户） */
    #[excel("操作类别", readConverterExp = "0=其它,1=后台用户,2=手机端用户")]
    pub operator_type: Option<u16>,
    /** 操作人员 */
    #[excel("操作人员")]
    pub oper_name: Option<String>,
    /** 部门名称 */
    #[excel("部门名称")]
    pub dept_name: Option<String>,
    /** 请求url */
    #[excel("请求地址")]
    pub oper_url: Option<String>,
    /** 操作地址 */
    #[excel("操作地址")]
    pub oper_ip: Option<String>,
    /** 操作地点 */
    #[excel("操作地点")]
    pub oper_location: Option<String>,
    /** 请求参数 */
    #[excel("请求参数")]
    pub oper_param: Option<String>,
    /** 返回参数 */
    #[excel("返回参数")]
    pub json_result: Option<String>,
    /** 操作状态（0正常 1异常） */
    #[excel("状态", readConverterExp = "0=正常,1=异常")]
    pub status: Option<char>,
    /** 错误消息 */
    #[excel("错误消息")]
    pub error_msg: Option<String>,
    /** 操作时间 */
    #[serde(with = "crate::utils::date_time_format")]
    #[excel("操作时间", width = 30.0)]
    pub oper_time: Option<DateTime>,
    /** 消耗时间 */
    //todo , suffix = "毫秒"
    #[excel("消耗时间")]
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
