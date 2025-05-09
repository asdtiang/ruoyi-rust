use crate::gen::domain::mapper::gen_table;
use crate::gen::domain::mapper::gen_table::{select_db_table_list, GenTable, TablePageDTO};
use crate::gen::domain::mapper::gen_table_column::{
    select_db_table_columns_by_name, GenTableColumn,
};
use crate::error::Error;
use crate::error::Result;
use crate::web_data::get_user_name;
use crate::{pool, remove_batch};
use macros::transactional;
use minijinja::syntax::SyntaxConfig;
use rbatis::object_id::ObjectId;
use rbatis::{field_name, Page, PageRequest};
use std::fs;
use crate::gen::domain::dto::GenTableUpdateDTO;
use crate::gen::domain::vo::table::GenTableGenVO;
use crate::gen::domain::vo::table_column::GenTableColumnGenVO;
use crate::gen::GEN_CONTEXT;
use crate::gen::service::{gen_constants, gen_utils, jinja_utils};

/// table service
pub struct GenTableService {}

impl GenTableService {
    pub async fn page(&self, arg: &TablePageDTO) -> Result<Page<GenTable>> {
        let data = GenTable::select_page(pool!(), &PageRequest::from(arg), arg).await?;

        Ok(data)
    }
    pub async fn list_all(&self) -> Result<Vec<GenTable>> {
        let data = GenTable::select_all(pool!()).await?;
        Ok(data)
    }
    pub async fn db_list_page(&self, arg: &TablePageDTO) -> Result<Page<GenTable>> {
        let data = select_db_table_list(pool!(), &PageRequest::from(arg), arg).await?;

        Ok(data)
    }
    pub async fn detail(&self, table_id: &str) -> Result<GenTable> {
        let table = GenTable::select_by_column(pool!(), field_name!(GenTable.table_id), table_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} ！", table_id)))?;
        Ok(table)
    }

    // pub async fn add(&self, dto: GenTableAddDTO) -> Result<u64> {
    //     let mut data = GenTable::from(dto);
    //     data.create_by = Some(crate::web_data::get_user_name());
    //     let result = Ok(GenTable::insert(pool!(), &data).await?.rows_affected);
    //     result
    // }

    #[transactional]
    pub async fn update(&self, dto: GenTableUpdateDTO) -> Result<u64> {
        let columns = dto.columns.clone();
        let mut data = GenTable::from(dto);

        data.update_by = Some(crate::web_data::get_user_name());
        let result = GenTable::update_by_column(&tx, &data, "table_id").await?;
        match columns {
            None => {}
            Some(list) => {
                for column in list {
                    let column = GenTableColumn::from(column);
                    GenTableColumn::update_by_column(&tx, &column, "column_id").await?;
                }
            }
        }
        Ok(result.rows_affected)
    }

    #[transactional]
    pub async fn remove(&self, table_id: &str) -> Result<u64> {
       // let targets = GenTable::select_by_column(&tx, "table_id", table_id).await?;

        let r = GenTable::delete_by_column(&tx, "table_id", table_id).await?;
        if r.rows_affected > 0 {
            GenTableColumn::delete_by_column(&tx, "table_id", table_id).await?;
        }
        Ok(r.rows_affected)
    }
    remove_batch!(table_ids);

    #[transactional]
    pub async fn import_gen_table(&self, table_name_list: Vec<&str>) -> Result<Vec<GenTable>> {
        let tables = gen_table::select_db_table_list_by_names(&tx, &table_name_list).await?;
        let mut res = vec![];
        let oper_name = get_user_name();
        for mut table in tables {
            let table_name = table.table_name.clone().unwrap_or_default();
            table.table_id = Some(ObjectId::new().to_string());
            gen_utils::init_table(&mut table, &oper_name);
            let r = GenTable::insert(&tx, &table).await?;
            if r.rows_affected > 0 {
                // 保存列信息
                let gen_table_columns = select_db_table_columns_by_name(&tx, &table_name).await?;
                for mut column in gen_table_columns {
                    column.column_id = Some(ObjectId::new().to_string());
                    gen_utils::init_column_field(&mut column, &table);
                    GenTableColumn::insert(&tx, &column).await?;
                }
            }
            res.push(table);
        }

        Ok(res)
    }
    pub async fn generator_code(&self, table_names: Vec<&str>) -> Result<()> {
        for table_name in table_names {
            let gen_table = GenTable::select_by_column(pool!(), "table_name", table_name)
                .await?
                .into_iter()
                .next();

            if let Some(gen_table) = gen_table {
                let columns = GEN_CONTEXT
                    .gen_table_column_service
                    .select_gen_table_column_list_by_table_id(
                        &gen_table.table_id.clone().unwrap_or_default(),
                    )
                    .await?;
                let mut insert_edit_cnt=0;
                let columns = columns
                    .into_iter()
                    .map(|c| { 
                        if c.is_edit.is_some_and(|b|b==gen_constants::REQUIRE) ||c.is_insert.is_some_and(|b|b==gen_constants::REQUIRE){
                            insert_edit_cnt=insert_edit_cnt+1;
                        }
                        GenTableColumnGenVO::from(c) })
                    .collect::<Vec<_>>();

                use minijinja::{Environment};
                let mut env = Environment::new();
                env.set_syntax(
                    SyntaxConfig::builder()
                       // .block_delimiters("{%", "%}")
                        .variable_delimiters("${{", "}}")
                        .comment_delimiters("{#", "#}")
                        .build()
                        .unwrap(),
                );

                let rv = std::env::current_dir()?
                    .join("template")
                    .join("rust")
                    .join("dto.rs.jinja");
                let contents = fs::read(&rv)
                    .ok()
                    .map(|bs| String::from_utf8(bs).unwrap_or_default())
                    .unwrap_or_default();
                let mut  table=GenTableGenVO::from(gen_table);
                table.insert_edit_cnt=insert_edit_cnt;
                env.add_template("controller", &contents).unwrap();

                let tmpl = env.get_template("controller").unwrap();
                let ctx =jinja_utils::prepare_context(table,columns);

                let  code=tmpl.render(ctx).unwrap();
                println!("{}", code);
               // println!("{}", prettyplease::unparse(&syn::parse_file(&code).unwrap()));
                // use markup_fmt::{config::FormatOptions, format_text, Language};
                //
                // let mut options = FormatOptions::default();
                // let mut len=code.len();
                // loop {
                //     println!("len:{len}");
                //      code= code.replace("\r", "").replace("\n\n", "\n");
                //     if code.len() == len {
                //         break;
                //     }
                //     len=code.len();
                //
                // }
                // let s=   format_text(&code,Language::Vue,&options,|code, _| {
                //     Ok::<_, ()>(code.into())
                // });
                // println!("{}",s.unwrap_or_default());
            }
        }
        Ok(())
    }
}
