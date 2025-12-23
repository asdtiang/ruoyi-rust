use macros::{replace_pool, transactional};
use rbatis::{field_name, Page, PageRequest};
use rbs::to_value;

use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::system::domain::dto::DictTypePageDTO;
use crate::system::domain::mapper::sys_dict_type::SysDictType;
use crate::system::domain::vo::SysDictTypeVO;
use crate::{check_unique, export_excel_service, pool, remove_batch_tx};

pub struct SysDictTypeService {}

impl SysDictTypeService {
    pub async fn page(&self, arg: &DictTypePageDTO) -> Result<Page<SysDictTypeVO>> {
        let data = SysDictType::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictTypeVO>::from(data);
        Ok(page)
    }
    pub async fn finds_all(&self) -> Result<Vec<SysDictTypeVO>> {
        let data = SysDictType::select_all(pool!()).await?;
        let res = data.into_iter().map(|d| SysDictTypeVO::from(d)).collect();
        Ok(res)
    }
    pub async fn detail(&self, dict_id: &str) -> Result<SysDictTypeVO> {
        let dict_type = SysDictType::select_by_column(pool!(), field_name!(SysDictType.dict_id), dict_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_id)))?;
        let dict_type_vo = SysDictTypeVO::from(dict_type);
        Ok(dict_type_vo)
    }
    pub async fn add(&self, arg: &SysDictType) -> Result<u64> {
        self.check_dict_type_unique(&None, arg.dict_type.clone().unwrap_or_default())
            .await?;
        let result = Ok(SysDictType::insert(pool!(), &arg).await?.rows_affected);
        let _ = CONTEXT.sys_dict_data_service.update_cache().await;
        result
    }
    #[transactional(tx)]
    pub async fn update(&self, data: SysDictType) -> Result<u64> {
        self.check_dict_type_unique(&data.dict_id, data.dict_type.clone().unwrap_or_default())
            .await?;

        let result = SysDictType::update_by_column(&tx, &data, "dict_id").await;
        if result.is_ok() {
            //更新dict_data
            CONTEXT.sys_dict_data_service.update_cache().await?;

            let dict_type_in_db = self.detail(&data.dict_id.clone().unwrap_or_default()).await?;
            if !data.dict_type.eq(&dict_type_in_db.dict_type) {
                let _ = &tx
                    .exec(
                        "update sys_dict_data set dict_type = '?' where dict_type = ?",
                        vec![
                            rbs::to_value!(data.dict_type),
                            rbs::to_value!(dict_type_in_db.dict_type),
                        ],
                    )
                    .await?;
            }
        }
        Ok(result?.rows_affected)
    }
  #[replace_pool]
    pub async fn remove(&self, dict_id: &str) -> Result<u64> {
        let targets = SysDictType::select_by_column(pool!(), "dict_id", dict_id).await?;
        if targets.len() == 1 {
            let dict_type = targets.get(0).unwrap().dict_type.clone().unwrap();
            let count: u64 = pool!()
                .query_decode(
                    "select count(1) as count from sys_dict_data where dict_type =?",
                    vec![to_value!(dict_type)],
                )
                .await?;
            if count > 0 {
                return Err(Error::from("存在子项,不允许删除！"));
            }
        } else {
            return Err(Error::from(format!("字典id{}不存在！", dict_id)));
        }

        let r = SysDictType::delete_by_column(pool!(), "dict_id", dict_id).await?;
        if r.rows_affected > 0 {
            CONTEXT.sys_dict_data_service.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_type", &targets).await?;
        }
        Ok(r.rows_affected)
    }
    check_unique!(
        check_dict_type_unique,
        "sys_dict_type",
        dict_id,
        dict_type,
        "字典已存在"
    );
    remove_batch_tx!(dict_type_ids);
    export_excel_service!(DictTypePageDTO, SysDictTypeVO, SysDictType::select_page);
}
