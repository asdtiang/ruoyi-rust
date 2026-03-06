use crate::ap::domain::dto::ApProductBrandPageDTO;
use crate::ap::domain::mapper::ap_product_brand::ApProductBrand;
use crate::ap::domain::vo::ApProductBrandListVO;
use crate::error::Error;
use rbatis::{Page, PageRequest};
use crate::error::Result;
use crate::{export_excel_service, pool, remove_batch_tx};
pub struct ApProductBrandService {}
impl ApProductBrandService {
    pub async fn page(&self, arg: &ApProductBrandPageDTO) -> Result<Page<ApProductBrand>> {
        let data = ApProductBrand::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }
    pub async fn detail(&self, id: &str) -> Result<ApProductBrand> {
        let product_brand = ApProductBrand::select_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{} ！", id)))?;
        Ok(product_brand)
    }
    pub async fn add(&self, product_brand: ApProductBrand) -> Result<u64> {
        let result = Ok(ApProductBrand::insert(pool!(), &product_brand).await?.rows_affected);
        result
    }
    pub async fn update(&self, product_brand: ApProductBrand) -> Result<u64> {
        let result =
            ApProductBrand::update_by_map(pool!(), &product_brand, rbs::value! {"id": product_brand.id.clone()})
                .await?;
        Ok(result.rows_affected)
    }
    #[macros::replace_pool]
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let r = ApProductBrand::delete_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .rows_affected;
        Ok(r)
    }
    remove_batch_tx!(ids);
    export_excel_service!(ApProductBrandPageDTO, ApProductBrandListVO, ApProductBrand::select_page);
}
