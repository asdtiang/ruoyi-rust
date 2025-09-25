use crate::gen::domain::dto::{GenTableUpdateDTO, TableNamesDTO};
use crate::gen::domain::mapper::gen_table::{GenTable, TablePageDTO};
use crate::gen::domain::mapper::gen_table_column::GenTableColumn;
use crate::gen::domain::vo::table::GenTableVO;
use crate::gen::domain::vo::table_column::GenTableColumnVO;
use crate::gen::GEN_CONTEXT;
use crate::{update_marco, PageVO, RespJson, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;
use rbatis::Page;
use serde_json::json;
use std::collections::HashMap;

#[axum::debug_handler]
pub async fn list(user_cache: crate::UserCache, dto: Json<TablePageDTO>) -> impl IntoResponse {
    match crate::web::check_permit(&user_cache, "tool:gen:list").await {
        None => {
            let data = GEN_CONTEXT.gen_table_service.page(&dto.0).await;
            let data = data.map(|d| Page::<GenTableVO>::from(d));
            PageVO::from_result(&data).into_response()
        }
        Some(res) => { res.into_response() }
    }
}

#[pre_authorize("tool:gen:list")]
pub async fn db_list(dto: Json<TablePageDTO>) -> impl IntoResponse {
    let data = GEN_CONTEXT.gen_table_service.db_list_page(&dto.0).await;
    let data = data.map(|d| Page::<GenTableVO>::from(d));
    PageVO::from_result(&data).into_response()
}

#[pre_authorize("tool:gen:query")]
pub async fn detail(table_id: Path<String>) -> impl IntoResponse {
    let table = GEN_CONTEXT.gen_table_service.detail(&table_id.0).await;

    let table = table.map(|d| GenTableVO::from(d));

    let all_tables = GEN_CONTEXT.gen_table_service.list_all().await;
    let all_tables = all_tables.map(|d| d.into_iter().map(|d| GenTableVO::from(d)).collect::<Vec<_>>());
    let list = GEN_CONTEXT
        .gen_table_column_service
        .select_gen_table_column_list_by_table_id(&table_id.0)
        .await;
    let list = list.map(|d| d.into_iter().map(|d| GenTableColumnVO::from(d)).collect::<Vec<_>>());
    let mut res = RespJson::success();
    let mut data = HashMap::new();

    data.insert("info".to_string(), json!(table.unwrap())); //fixme
    data.insert("rows".to_string(), json!(list.unwrap_or_default()));
    data.insert("tables".to_string(), json!(all_tables.unwrap_or_default()));
    res.insert("data".to_string(), json!(data));
    // return success(map);
    res.into_response()
}

#[pre_authorize("tool:gen:edit", user_cache)]
pub async fn update(dto: Json<GenTableUpdateDTO>) -> impl IntoResponse {
    let columns = dto.0.columns.clone().map(|d| {
        d.into_iter()
            .map(|d| {
                let mut col = GenTableColumn::from(d);
                col.update_by = Some(user_cache.user_name());
                col.update_time = Some(rbatis::rbdc::datetime::DateTime::now().set_nano(0).into());
                col
            })
            .collect::<Vec<_>>()
    });
    update_marco!(data, dto, user_cache, GenTable);
    let res = GEN_CONTEXT.gen_table_service.update(data, columns).await;
    RespVO::<u64>::judge_result(res, "保存成功！", "保存失败！").into_response()
}

#[pre_authorize("tool:gen:remove")]
pub async fn remove(table_id: Path<String>) -> impl IntoResponse {
    let rows_affected = GEN_CONTEXT.gen_table_service.remove_batch(&table_id.0).await;
    RespVO::<u64>::judge_result(rows_affected, "", "删除失败！").into_response()
}

#[pre_authorize("tool:gen:import", user_cache)]
pub async fn import_table(table_name: axum::extract::Query<TableNamesDTO>) -> impl IntoResponse {
    let tables = table_name.0.tables.unwrap_or_default();
    let table_names = tables.split(",").collect::<Vec<&str>>();
    let insert_cnt = GEN_CONTEXT.gen_table_service.import_gen_table(table_names,&user_cache.user_name).await;

    RespVO::<u64>::judge_result(insert_cnt, "导入成功！", "导入失败！").into_response()
}

#[pre_authorize("tool:gen:code")]
pub async fn batch_gen_code(table_name: Path<String>) -> impl IntoResponse {
    let tables = table_name.0;
    let table_names = tables.split(",").collect::<Vec<&str>>();
    let tables = GEN_CONTEXT.gen_table_service.generate_code(table_names).await;
    RespVO::from_result(&tables).into_response()
}
#[pre_authorize("tool:gen:code")]
pub async fn preview_code(table_id: Path<String>) -> impl IntoResponse {
    let table_id = table_id.0;
    let codes = GEN_CONTEXT.gen_table_service.preview_code(&table_id).await;
    RespVO::from_result(&codes).into_response()
}
#[pre_authorize("tool:gen:code")]
pub async fn synch_db(table_name: Path<String>) -> impl IntoResponse {
    let tables = GEN_CONTEXT.gen_table_service.synch_db(&table_name.0).await;
    RespVO::from_result(&tables).into_response()
}
