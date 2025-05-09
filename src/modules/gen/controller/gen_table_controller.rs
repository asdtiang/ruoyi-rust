use crate::gen::domain::dto::{GenTableUpdateDTO, TableNamesDTO};
use crate::gen::domain::mapper::gen_table::TablePageDTO;
use crate::gen::domain::vo::table::GenTableVO;
use crate::gen::domain::vo::table_column::GenTableColumnVO;
use crate::{PageVO, RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum:: Json;
use macros::pre_authorize;
use rbatis::Page;
use serde_json::json;
use std::collections::HashMap;
use crate::gen::GEN_CONTEXT;

//#[get("/table/list")]
#[pre_authorize("tool:gen:list")]
pub async fn list(dto: Json<TablePageDTO>) -> impl IntoResponse {
    let data = GEN_CONTEXT.gen_table_service.page(&dto.0).await;
    let data = data.map(|d| Page::<GenTableVO>::from(d));
    PageVO::from_result(&data).into_response()
}

//#[get("/table/list")]
#[pre_authorize("tool:gen:list")]
pub async fn db_list(dto: Json<TablePageDTO>) -> impl IntoResponse {
    let data = GEN_CONTEXT.gen_table_service.db_list_page(&dto.0).await;
    let data = data.map(|d| Page::<GenTableVO>::from(d));
    PageVO::from_result(&data).into_response()
}
//#[get("/table/{table_id}")]
#[pre_authorize("tool:gen:query")]
pub async fn detail(table_id: Path<String>) -> impl IntoResponse {
    let table = GEN_CONTEXT.gen_table_service.detail(&table_id.0).await;

    let table = table.map(|d| GenTableVO::from(d));

    let all_tables = GEN_CONTEXT.gen_table_service.list_all().await;
    let all_tables = all_tables.map(|d| {
        d.into_iter()
            .map(|d| GenTableVO::from(d))
            .collect::<Vec<_>>()
    });
    let list = GEN_CONTEXT
        .gen_table_column_service
        .select_gen_table_column_list_by_table_id(&table_id.0)
        .await;
    let list = list.map(|d| {
        d.into_iter()
            .map(|d| GenTableColumnVO::from(d))
            .collect::<Vec<_>>()
    });
    // GenTable table = genTableService.selectGenTableById(tableId);
    // List<GenTable> tables = genTableService.selectGenTableAll();
    // List<GenTableColumn> list = genTableColumnService.select_gen_table_column_list_by_table_id(tableId);
    // Map<String, Object> map = new HashMap<String, Object>();
    let mut res = RespJson::success();
    let mut data = HashMap::new();

    data.insert("info".to_string(), json!(table.unwrap())); //fixme
    data.insert("rows".to_string(), json!(list.unwrap_or_default()));
    data.insert("tables".to_string(), json!(all_tables.unwrap_or_default()));
    res.insert("data".to_string(), json!(data));
    // return success(map);
    res.into_response()
}

//#[put("/table")]
#[pre_authorize("tool:gen:edit")]
pub async fn update(dto: Json<GenTableUpdateDTO>) -> impl IntoResponse {
    let res = GEN_CONTEXT.gen_table_service.update(dto.0).await;
    RespVO::from_result(&res).into_response()
}

//#[delete("/table/{table_id}")]
#[pre_authorize("tool:gen:remove")]
pub async fn remove(table_id: Path<String>) -> impl IntoResponse {
    let rows_affected = GEN_CONTEXT
        .gen_table_service
        .remove_batch(&table_id.0)
        .await;
    RespVO::<u64>::judge_result(&rows_affected, "", "删除失败！").into_response()
}

#[pre_authorize("tool:gen:import")]
pub async fn import_table(table_name: axum::extract::Query<TableNamesDTO>) -> impl IntoResponse {
    let tables = table_name.0.tables.unwrap_or_default();
    let table_names = tables.split(",").collect::<Vec<&str>>();
    let tables = GEN_CONTEXT
        .gen_table_service
        .import_gen_table(table_names)
        .await;
    let tables = tables.map(|list| {
        list.into_iter()
            .map(|gt| GenTableVO::from(gt))
            .collect::<Vec<_>>()
    });
    RespVO::from_result(&tables).into_response()
}

#[pre_authorize("tool:gen:code")]
pub async fn batch_gen_code(table_name: axum::extract::Query<TableNamesDTO>) -> impl IntoResponse {
    let tables = table_name.0.tables.unwrap_or_default();
    let table_names = tables.split(",").collect::<Vec<&str>>();
    let tables = GEN_CONTEXT
        .gen_table_service
        .generator_code(table_names)
        .await;
    RespVO::from_result(&tables).into_response()
}
