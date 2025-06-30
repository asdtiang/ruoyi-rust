use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysLogininfor {});


impl_select_page!(SysLogininfor{select_page(dto: &crate::system::domain::dto::LogininforPageDTO) =>
    "``
      if do_count == false:
         ` order by login_time desc`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysLogininfor {
    /** ID */
    pub info_id: Option<String>,
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
    /** 提示消息 */
    pub msg: Option<String>,
    /** 访问时间 */
    pub login_time: Option<DateTime>,
}