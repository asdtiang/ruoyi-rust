use crate::error::Result;
use crate::system::domain::dto::OperLogPageDTO;
use crate::system::domain::mapper::sys_oper_log::SysOperLog;
use crate::system::domain::vo::SysOperLogVO;
use crate::{export_excel_service, pool, remove_batch_tx};
use macros::replace_pool;
use rbatis::{Page, PageRequest};

pub struct SysOperLogService {}

impl SysOperLogService {
    pub async fn page(&self, arg: &OperLogPageDTO) -> Result<Page<SysOperLog>> {
        let data = SysOperLog::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }

    //异步加入日志
    pub async fn add_async(&self, arg: &SysOperLog) -> Result<u64> {
        let info = arg.to_owned();
        tokio::spawn(async move {
            let _ = SysOperLog::insert(pool!(), &info).await;
        });
        Ok(1)
    }

    #[replace_pool]
    pub async fn remove(&self, oper_id: &str) -> Result<u64> {
        let r = SysOperLog::delete_by_column(pool!(), "oper_id", oper_id).await?;
        Ok(r.rows_affected)
    }
    pub async fn clean(&self) -> Result<u64> {
        let res = pool!().exec("delete from sys_oper_log", vec![]).await?;
        Ok(res.rows_affected)
    }
    remove_batch_tx!(log_ids);
    export_excel_service!(OperLogPageDTO, SysOperLogVO, SysOperLog::select_page);
}
