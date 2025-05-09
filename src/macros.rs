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