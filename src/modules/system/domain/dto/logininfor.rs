use macros::page_request;
use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogininforPageDTO {
    /** 用户账号 */
    pub user_name: Option<String>,
    /** 登录IP地址 */
    pub ipaddr: Option<String>,
    /** 登录地点 */
    pub login_location: Option<String>,
    /** 浏览器类型 */
    pub browser: Option<String>,
    /** 操作系统 */
    pub os: Option<String>,
    /** 登录状态 0成功 1失败 */
    pub status: Option<char>,
}

