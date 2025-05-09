use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysLogininfor {});


impl_select_page!(SysLogininfor{select_page(dto: &crate::system::domain::dto::LogininforPageDTO) =>
    "``
      if do_count == false:
         ` order by login_time desc`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysLogininfor {
    pub info_id: Option<String>,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<char>,
    pub msg: Option<String>,
    pub login_time: Option<DateTime>,
}