use crate::ap::domain::dto::ApProductUnitPageDTO;
use crate::ap::domain::mapper::ap_product_unit::ApProductUnit;
use crate::ap::domain::vo::ApProductUnitListVO;
use crate::error::Error;
use rbatis::{Page, PageRequest};
use crate::error::Result;
use crate::{export_excel_service, pool, remove_batch_tx};
pub struct ApProductUnitService {}
impl ApProductUnitService {
    pub async fn page(&self, arg: &ApProductUnitPageDTO) -> Result<Page<ApProductUnit>> {
        let data = ApProductUnit::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }
    pub async fn detail(&self, id: &str) -> Result<ApProductUnit> {
        let product_unit = ApProductUnit::select_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{} ！", id)))?;
        Ok(product_unit)
    }
    pub async fn add(&self, product_unit: ApProductUnit) -> Result<u64> {
        let result = Ok(ApProductUnit::insert(pool!(), &product_unit).await?.rows_affected);
        result
    }
    pub async fn update(&self, product_unit: ApProductUnit) -> Result<u64> {
        let result =
            ApProductUnit::update_by_map(pool!(), &product_unit, rbs::value! {"id": product_unit.id.clone()}).await?;
        Ok(result.rows_affected)
    }
    #[macros::replace_pool]
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let r = ApProductUnit::delete_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .rows_affected;
        Ok(r)
    }
    remove_batch_tx!(ids);
    export_excel_service!(ApProductUnitPageDTO, ApProductUnitListVO, ApProductUnit::select_page);
}
