use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::system::domain::dto::{NoticeAddDTO, NoticePageDTO, NoticeUpdateDTO};
use crate::system::domain::mapper::sys_notice::SysNotice;
use crate::system::domain::vo::SysNoticeVO;
use crate::system::service::dict_utils::get_dict_label_default;
use crate::{pool, remove_batch};
use rbatis::{field_name, Page, PageRequest};
use rust_xlsxwriter::{ColNum, Format, Workbook};

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

    pub async fn export(&self, arg: &NoticePageDTO) -> Result<Vec<u8>> {
        let mut dto = arg.clone();
        dto.page_size = Some(u64::MAX);
        let mut res = Vec::new();
        loop {
            let data = SysNotice::select_page(pool!(), &PageRequest::from(arg), arg).await?;
            data.records
                .into_iter()
                .for_each(|r| res.push(SysNoticeVO::from(r)));
            if data.page_size * data.page_no >= data.total {
                break;
            }
            dto.page_no = dto.page_no.map(|p| p + 1);
        }
        let excel_attrs = SysNoticeVO::get_excel_attr();

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // Add formats
        let bold_format = Format::new().set_bold();
        //let money_format = Format::new().set_num_format("$#,##0.00");

        for (i, attr) in excel_attrs.iter().enumerate() {
            // Write headers
            worksheet.write_string_with_format(0, i as ColNum, attr.name.clone(), &bold_format)?;
        }
        // Write data
        for (i, vo) in res.iter().enumerate() {
            let row = i as u32 + 1;
            let values = serde_json::json!(vo);
            for (col, attr) in excel_attrs.iter().enumerate() {
                let value = match values.get(&attr.camel_case_indent) {
                    None => &attr.default_value.clone().unwrap_or_default(),
                    Some(e) => { 
                        e.as_str().unwrap_or("not") },
                };

                if let Some(dict_type) = attr.dict_type.clone() {
                    let value = &get_dict_label_default(&dict_type, value).await?;
                    worksheet.write_string(row, col as ColNum, value)?;
                } else {
                    worksheet.write_string(row, col as ColNum, value)?;
                }
            }
        }

        Ok(workbook.save_to_buffer()?)
    }
}
