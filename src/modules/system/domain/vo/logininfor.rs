use crate::system::domain::mapper::sys_logininfor::SysLogininfor;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive( macros::Export)]
pub struct SysLogininforVO {
    /** ID */
    #[excel( "序号")]
    pub info_id: Option<String>,
    /** 用户账号 */
    #[excel( "用户账号")]
    pub user_name: Option<String>,
    /** 登录IP地址 */
    #[excel( "登录地址")]
    pub ipaddr: Option<String>,
    /** 登录地点 */
    #[excel( "登录地点")]
    pub login_location: Option<String>,
    /** 浏览器类型 */
    #[excel( "浏览器")]
    pub browser: Option<String>,
    /** 操作系统 */
    #[excel( "操作系统")]
    pub os: Option<String>,
    /** 登录状态 0成功 1失败 */
    #[excel( "登录状态", readConverterExp = "0=成功,1=失败")]
    pub status: Option<char>,
    /** 提示消息 */
    #[excel( "提示消息")]
    pub msg: Option<String>,
    /** 访问时间 */
    #[serde(with = "crate::utils::date_time_format")]
    #[excel( "访问时间", width = 30.0)]
    pub login_time: Option<DateTime>,
}

impl From<SysLogininfor> for SysLogininforVO {
    fn from(arg: SysLogininfor) -> Self {
        Self {
            info_id: arg.info_id,
            user_name: arg.user_name,
            ipaddr: arg.ipaddr,
            login_location: arg.login_location,
            browser: arg.browser,
            os: arg.os,
            status: arg.status,
            msg: arg.msg,
            login_time: arg.login_time
        }
    }
}
