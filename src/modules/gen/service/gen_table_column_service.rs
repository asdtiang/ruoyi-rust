use crate::context::CONTEXT;
use crate::error::Result;
use crate::gen::domain::mapper::gen_table_column::GenTableColumn;
use crate::{pool, remove_batch};

/// table service
pub struct GenTableColumnService {}

impl GenTableColumnService {
    pub async fn list_all(&self) -> Result<Vec<GenTableColumn>> {
        let data = GenTableColumn::select_all(pool!()).await?;
        Ok(data)
    }

    pub async fn remove(&self, table_id: &str) -> Result<u64> {
        let targets = GenTableColumn::select_by_column(pool!(), "table_id", table_id).await?;

        let r = GenTableColumn::delete_by_column(pool!(), "table_id", table_id).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_table", &targets).await?;
        }
        Ok(r.rows_affected)
    }
    remove_batch!(table_ids);

    pub async fn select_gen_table_column_list_by_table_id(&self, table_id: &str) -> Result<Vec<GenTableColumn>> {
        let list = GenTableColumn::select_columns_by_table_id(pool!(), table_id).await?;
        Ok(list)
    }
}
