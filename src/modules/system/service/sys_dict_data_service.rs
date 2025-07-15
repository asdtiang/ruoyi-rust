use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::system::domain::dto::DictDataPageDTO;
use crate::system::domain::mapper::sys_dict_data::SysDictData;
use crate::system::domain::vo::{SysDictDataSimpleVO, SysDictDataVO};
use crate::system::service::dict_utils::get_dict_redis_key;
use crate::{check_unique, export_excel_service, pool, remove_batch};
use rbatis::{field_name, Page, PageRequest};
use rbs::to_value;
use std::collections::HashMap;

pub struct SysDictDataService {}

impl SysDictDataService {
    pub async fn page(&self, arg: &DictDataPageDTO) -> Result<Page<SysDictDataVO>> {
        let data = SysDictData::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<SysDictDataVO>::from(data);
        Ok(page)
    }

    pub async fn get_by_dict_type(&self, dict_type: &String) -> Result<Vec<SysDictDataSimpleVO>> {
        let data = SysDictData::select_by_dict_type(pool!(), &dict_type).await?;
        let res=   data.into_iter().map(|d| SysDictDataSimpleVO::from(d)).collect();
        Ok(res)
    }

    pub async fn detail(&self, dict_code: &str) -> Result<SysDictDataVO> {
        let dict_data = SysDictData::select_by_column(pool!(), field_name!(SysDictData.dict_code), dict_code)
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} ", dict_code)))?;
        let dict_data_vo = SysDictDataVO::from(dict_data);
        Ok(dict_data_vo)
    }

    pub async fn add(&self, arg: &SysDictData) -> Result<u64> {
        self.check_dict_value_unique(&arg.dict_code.clone(),&arg.dict_type.clone().unwrap_or_default(),&arg.dict_value.clone().unwrap_or_default()).await?;
        let result = Ok(SysDictData::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        result
    }

    pub async fn update(&self, data: SysDictData) -> Result<u64> {
        self.check_dict_value_unique(&data.dict_code.clone(),&data.dict_type.clone().unwrap_or_default(),&data.dict_value.clone().unwrap_or_default()).await?;
        let result = SysDictData::update_by_column(pool!(), &data, "dict_code").await;
        if result.is_ok() {
            self.update_cache().await?;
        }
        Ok(result?.rows_affected)
    }

    pub async fn remove(&self, dict_code: &str) -> Result<u64> {
        let targets = SysDictData::select_by_column(pool!(), "dict_code", dict_code).await?;

        let r = SysDictData::delete_by_column(pool!(), "dict_code", dict_code).await?;
        if r.rows_affected > 0 {
            self.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_data", &targets).await?;
        }
        Ok(r.rows_affected)
    }

    /// update for all cache
    pub async fn update_cache(&self) -> Result<()> {
        let mut all = SysDictData::select_all(pool!()).await?;
        all.sort_by(|a, b| a.dict_sort.cmp(&b.dict_sort));
        let mut dict_data_map: HashMap<String, Vec<SysDictDataSimpleVO>> = HashMap::new();
        for dict_data in all {
            let key = dict_data.dict_type.clone().unwrap_or_default();
            let data_sim = SysDictDataSimpleVO::from(dict_data);
            if dict_data_map.contains_key(&key) {
                dict_data_map.get_mut(&key).unwrap().push(data_sim);
            } else {
                dict_data_map.insert(key, vec![data_sim]);
            }
        }
        for (key, sims) in dict_data_map {
            CONTEXT.cache_service.set_json(&get_dict_redis_key(&key), &sims).await?;
        }
         
        Ok(())
    }


    check_unique!(check_dict_value_unique,"sys_dict_data",dict_code, dict_type, dict_value,"字典数据键值已存在");
    remove_batch!(dict_data_ids);

    export_excel_service!(DictDataPageDTO, SysDictDataVO,SysDictData::select_page);
}
