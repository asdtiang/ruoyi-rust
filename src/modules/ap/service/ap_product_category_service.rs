use log::info;
use crate::ap::domain::dto::ApProductCategoryPageDTO;
use crate::ap::domain::mapper::ap_product_category::ApProductCategory;
use crate::ap::domain::vo::ApProductCategoryListVO;
use crate::error::Error;
use rbatis::{Page, PageRequest};
use crate::error::Result;
use crate::{export_excel_service, pool, remove_batch_tx};
use crate::system::domain::mapper::sys_menu::SysMenu;

pub struct ApProductCategoryService {}


impl ApProductCategoryService {
    pub async fn page(&self, arg: &ApProductCategoryPageDTO) -> Result<Page<ApProductCategory>> {
        let data = ApProductCategory::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }

    pub async fn list_parent(&self) -> Result<Vec<ApProductCategory>> {
        let res: Option<Vec<ApProductCategory>> = pool!().query_decode("
      select *
		from ap_product_category where parent_id is null
		order by order_num asc
       ", vec![])
            .await?;
        Ok(res.unwrap_or_default())
    }
    pub async fn detail(&self, id: &str) -> Result<ApProductCategory> {
        let product_category = ApProductCategory::select_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{} ！", id)))?;
        Ok(product_category)
    }
    pub async fn add(&self, mut product_category: ApProductCategory) -> Result<u64> {
        if product_category.parent_id.is_none() {
            let parent_ = ApProductCategory::select_by_map(pool!(), rbs::value! {"parent_id": product_category.parent_id})
				.await?
				.into_iter()
				.next()
				.ok_or_else(|| Error::from(format!("不存在:{} ！", 0)))?;
            product_category.parent_name=parent_.name;
		}
        let result = Ok(ApProductCategory::insert(pool!(), &product_category)
            .await?
            .rows_affected);
        result
    }
    pub async fn update(&self, mut product_category: ApProductCategory) -> Result<u64> {
        if product_category.parent_id.is_some() {
            let parent_ = ApProductCategory::select_by_map(pool!(), rbs::value! {"id": product_category.parent_id})
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| Error::from(format!("不存在:{} ！", 0)))?;
            info!("parent_:{:?}",parent_);
            product_category.parent_name=parent_.name;
        }
        info!("product_category:{:?}",product_category);
        let result = ApProductCategory::update_by_map(
            pool!(),
            &product_category,
            rbs::value! {"id": product_category.id.clone()},
        )
        .await?;
        Ok(result.rows_affected)
    }
    #[macros::replace_pool]
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let r = ApProductCategory::delete_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .rows_affected;
        Ok(r)
    }
    remove_batch_tx!(ids);
    export_excel_service!(
        ApProductCategoryPageDTO,
        ApProductCategoryListVO,
        ApProductCategory::select_page
    );
}
