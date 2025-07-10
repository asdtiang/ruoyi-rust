#[macro_export]
macro_rules! check_unique {
    ($func_name:ident, $table:expr, $id_col:ident, $key_col:ident,$hint:expr) => {
        pub async fn $func_name(&self, $id_col: &Option<String>, $key_col: &str) -> Result<()> {
            if $key_col.is_empty() {
                return Ok(());
            }
            let old_id: Option<String> = pool!()
                .query_decode(
                    &format!(
                        "select {} from {} where {} = ? limit 1",
                        stringify!($id_col),
                        $table,
                        stringify!($key_col)
                    ),
                    vec![to_value!($key_col)],
                )
                .await?;
            if old_id.eq($id_col) {
                Ok(())
            } else {
                Err(Error::from($hint))
            }
        }
    };
    ($func_name:ident, $table:expr, $id_col:ident, $key_col1:ident,$key_col2:ident,$hint:expr) => {
        pub async fn $func_name(
            &self,
            $id_col: &Option<String>,
            $key_col1: &str,
            $key_col2: &str,
        ) -> Result<()> {
            if $key_col1.is_empty() || $key_col2.is_empty() {
                return Ok(());
            }
            let old_id: Option<String> = pool!()
                .query_decode(
                    &format!(
                        "select {} from {} where {} = ? and {} = ? limit 1",
                        stringify!($id_col),
                        $table,
                        stringify!($key_col1),
                        stringify!($key_col2)
                    ),
                    vec![to_value!($key_col1), to_value!($key_col2)],
                )
                .await?;
            if old_id.eq($id_col) {
                Ok(())
            } else {
                Err(Error::from($hint))
            }
        }
    };
}

#[macro_export]
macro_rules! check_unique_sql {
    ($func_name:ident, $table:expr, $id_col:ident, $key_col:ident,$hint:expr,$sql:expr) => {
        pub async fn $func_name(&self, $id_col: &Option<String>, $key_col: &str) -> Result<()> {
            if $key_col.is_empty() {
                return Ok(());
            }
            let old_id: Option<String> = pool!()
                .query_decode($sql, vec![to_value!($key_col)])
                .await?;
            if old_id.eq($id_col) {
                Ok(())
            } else {
                Err(Error::from($hint))
            }
        }
    };
    ($func_name:ident, $table:expr, $id_col:ident, $key_col1:ident,$key_col2:ident,$hint:expr,$sql:expr) => {
        pub async fn $func_name(
            &self,
            $id_col: &Option<String>,
            $key_col1: &str,
            $key_col2: &str,
        ) -> Result<()> {
            if $key_col1.is_empty() || $key_col2.is_empty() {
                return Ok(());
            }
            let old_id: Option<String> = pool!()
                .query_decode($sql, vec![to_value!($key_col1), to_value!($key_col2)])
                .await?;
            if old_id.eq($id_col) {
                Ok(())
            } else {
                Err(Error::from($hint))
            }
        }
    };
}

#[macro_export]
macro_rules! get_config_value {
    ($key:expr)=> {
        $crate:::CONTEXT.sys_config_service.select_config_by_key($key).await.unwrap_or_default()
    };
}
#[macro_export]
macro_rules! remove_batch {
    ($ids:ident)=> {
         pub async fn remove_batch(&self,  $ids: &str) -> Result<u64> {//fixme 是否要加入事务
        let $ids=$ids.split(",").collect::<Vec<&str>>();
        for id in $ids {
            self.remove(id).await?;
        }
        Ok(1)
    }
    };
}

#[macro_export]
macro_rules! export_excel_controller {
    ($priv:expr,$page_dto:ident,$context:ident,$service:ident,$export_method:ident  )=> {
         #[pre_authorize($priv)]
        pub async fn export_to_excel(dto: Json<$page_dto>) -> impl IntoResponse {
            let bytes = $context.$service.$export_method(&dto.0).await;
            if let Ok(bytes) = bytes {
                // 设置响应头
                let mut headers = axum::http::HeaderMap::new();
                headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static(
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    )
                );
                headers.insert(
                    axum::http::header::CONTENT_DISPOSITION,
                    axum::http::HeaderValue::from_str("attachment; filename=\"export.xlsx\"").unwrap()
                );
                headers.insert(axum::http::header::CONTENT_LENGTH, axum::http::HeaderValue::from(bytes.len()));
                (headers, axum::body::Bytes::from(bytes)).into_response()
            } else {
                RespVO::<u64>::from_error_info(500, "导出错误！").into_response()
            }
        }
    };
}

#[macro_export]
macro_rules! export_excel_service {
    ($page_dto:ident,$entity_vo:ident,$page_method:path)=> {
          pub async fn export_as_excel_bytes(&self, arg: &$page_dto) -> Result<Vec<u8>> {
        let mut dto = arg.clone();
        dto.page_no=Some(1);
        dto.page_size = Some(u64::MAX);
        let mut res = Vec::new();
        loop {
            let data = $page_method(pool!(), &PageRequest::from(&dto), &dto).await?;
            data.records
                .into_iter()
                .for_each(|r| res.push($entity_vo::from(r)));
            if data.page_size * data.page_no >= data.total {
                break;
            }
            dto.page_no = dto.page_no.map(|p| p + 1);
        }

        crate::utils::excel_utils::to_excel(& res).await
    }
    };
}
//简化一下
#[macro_export]
macro_rules! router_with_handler {
    ($method:ident,$func:path,$($middle_func_list:ident),*)=> {
       $method($func)$(.route_layer(middleware::from_fn($middle_func_list)))*
    };
}