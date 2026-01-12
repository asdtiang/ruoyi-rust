use crate::error::Result;
use crate::system::domain::dto::LogininforPageDTO;
use crate::system::domain::mapper::sys_logininfor::SysLogininfor;
use crate::system::domain::vo::SysLogininforVO;
use crate::{export_excel_service, pool};
use rbatis::{Page, PageRequest};
use crate::context::CONTEXT;
use crate::utils::address_util;

pub struct SysLogininforService {}

impl SysLogininforService {
    pub async fn page(&self, arg: &LogininforPageDTO) -> Result<Page<SysLogininfor>> {
        let data = SysLogininfor::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }

    //异步加入日志
    pub async fn add_async(&self, ip:String,user_agent: String, username: String, status: char, msg: String) -> Result<u64> {
        tokio::spawn(async move {
            let address = if CONTEXT.config.address_enabled {
                     address_util::get_real_address_by_ip(&ip).await.ok()
            } else {
                None
            };
            let mut info=crate::utils::web_utils::build_logininfor(ip, user_agent, username, status, msg);
            info.login_location=address;
            let _=SysLogininfor::insert(pool!(), &info).await;
        });
        Ok(1)
    }


    pub async fn remove(&self, info_id: &str) -> Result<u64> {
        let r = SysLogininfor::delete_by_map(pool!(), rbs::value! {"info_id": info_id} ).await?;
        Ok(r.rows_affected)
    }
    pub async fn clean(&self) -> Result<u64> {
        let res =
            pool!().exec("delete from sys_logininfor",
                         vec![]).await.unwrap();
        Ok(res.rows_affected)
    }
    export_excel_service!(LogininforPageDTO, SysLogininforVO,SysLogininfor::select_page);
}
