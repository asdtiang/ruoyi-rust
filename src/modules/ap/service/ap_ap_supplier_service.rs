use crate::ap::domain::dto::ApSupplierPageDTO;
use crate::ap::domain::mapper::ap_ap_supplier::ApSupplier;
use crate::ap::domain::vo::ApSupplierListVO;
use crate::error::Error;
use rbatis::{Page, PageRequest};
use crate::error::Result;
use crate::{export_excel_service, pool, remove_batch_tx};
pub struct ApSupplierService {}
impl ApSupplierService {
    pub async fn page(&self, arg: &ApSupplierPageDTO) -> Result<Page<ApSupplier>> {
        let data = ApSupplier::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        Ok(data)
    }
    pub async fn detail(&self, id: &str) -> Result<ApSupplier> {
        let ap_supplier = ApSupplier::select_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{} ！", id)))?;
        Ok(ap_supplier)
    }
    pub async fn add(&self, ap_supplier: ApSupplier) -> Result<u64> {
        let result = Ok(ApSupplier::insert(pool!(), &ap_supplier).await?.rows_affected);
        result
    }
    pub async fn update(&self, ap_supplier: ApSupplier) -> Result<u64> {
        let result =
            ApSupplier::update_by_map(pool!(), &ap_supplier, rbs::value! {"id": ap_supplier.id.clone()}).await?;
        Ok(result.rows_affected)
    }
    #[macros::replace_pool]
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let r = ApSupplier::delete_by_map(pool!(), rbs::value! {"id": id})
            .await?
            .rows_affected;
        Ok(r)
    }
    remove_batch_tx!(ids);
    export_excel_service!(ApSupplierPageDTO, ApSupplierListVO, ApSupplier::select_page);
}
