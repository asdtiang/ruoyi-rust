use rbatis::{field_name, Page, PageRequest};

use crate::context::CONTEXT;
use  crate::system::domain::dto::{NoticeAddDTO, NoticePageDTO, NoticeUpdateDTO};
use  crate::system::domain::mapper::sys_notice::SysNotice;
use crate::error::Error;
use crate::error::Result;
use crate::{pool, remove_batch};

/// notice service
pub struct SysNoticeService {}

impl SysNoticeService {
    pub async fn page(&self, arg: &NoticePageDTO) -> Result<Page<SysNotice>> {
        let data = SysNotice::select_page(pool!(), &PageRequest::from(arg), arg).await?;

        Ok(data)
    }

    pub async fn detail(&self, notice_id: &str) -> Result<SysNotice> {
        let notice =
            SysNotice::select_by_column(pool!(), field_name!(SysNotice.notice_id), notice_id)
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| Error::from(format!("不存在:{:?} ！", notice_id)))?;
        Ok(notice)
    }

    pub async fn add(&self, dto: NoticeAddDTO) -> Result<u64> {
        let mut data = SysNotice::from(dto);
        data.create_by = Some(crate::web_data::get_user_name());
        let result = Ok(SysNotice::insert(pool!(), &data).await?.rows_affected);
        result
    }

    pub async fn update(&self, dto: NoticeUpdateDTO) -> Result<u64> {
        let mut data = SysNotice::from(dto);
        data.update_by = Some(crate::web_data::get_user_name());
        let result = SysNotice::update_by_column(pool!(), &data, "notice_id").await;
        Ok(result?.rows_affected)
    }

    pub async fn remove(&self, notice_id: &str) -> Result<u64> {
        let targets = SysNotice::select_by_column(pool!(), "notice_id", notice_id).await?;

        let r = SysNotice::delete_by_column(pool!(), "notice_id", notice_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add("sys_notice", &targets)
                .await?;
        }
        Ok(r.rows_affected)
    }
    remove_batch!(notice_ids);
}
