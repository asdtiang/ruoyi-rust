use crate::error::Error;
use crate::error::Result;
use crate::code_gen::domain::mapper::gen_table;
use crate::code_gen::domain::mapper::gen_table::{select_db_table_list, GenTable, TablePageDTO};
use crate::code_gen::domain::mapper::gen_table_column::{select_db_table_columns_by_name, GenTableColumn};
use crate::code_gen::service::{gen_constants, gen_utils, jinja_utils};
use crate::code_gen::GEN_CONTEXT;
use crate::utils::file_utils::find_files_with_extension;
use crate::utils::string::substring_unicode;
use crate::{pool, remove_batch_tx};
use macros::{replace_pool, transactional};
use minijinja::syntax::SyntaxConfig;
use rbatis::object_id::ObjectId;
use rbatis::{Page, PageRequest};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
        let table = GenTable::select_by_map(pool!(), rbs::value!{"table_id": table_id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{} ！", table_id)))?;
        Ok(table)
    }

    #[transactional(tx)]
    pub async fn update(&self, data: GenTable, columns: Option<Vec<GenTableColumn>>) -> Result<u64> {
        let result = GenTable::update_by_map(&tx, &data, rbs::value!{"table_id": data.table_id.clone()}).await?;
        match columns {
            None => {}
            Some(list) => {
                for column in list {
                    GenTableColumn::update_by_map(&tx, &column,rbs::value!{"column_id": column.column_id.clone()}).await?;
                }
            }
        }
        Ok(result.rows_affected)
    }
    #[replace_pool]
    pub async fn remove(&self, table_id: &str) -> Result<u64> {
        // let targets = GenTable::select_by_map(&tx, "table_id", table_id).await?;

        let r = GenTable::delete_by_map(tx, rbs::value!{"table_id": table_id}).await?;
        if r.rows_affected > 0 {
            GenTableColumn::delete_by_map(tx, rbs::value!{"table_id": table_id}).await?;
        }
        Ok(r.rows_affected)
    }
    remove_batch_tx!(table_ids);

    #[transactional(tx)]
    pub async fn import_gen_table(&self, table_name_list: Vec<&str>, oper_user_name: &str) -> Result<u64> {
        let tables = gen_table::select_db_table_list_by_names(&tx, &table_name_list).await?;
        for mut table in tables {
            let table_name = table.table_name.clone().unwrap_or_default();
            table.table_id = Some(ObjectId::new().to_string());
            gen_utils::init_table(&mut table, &oper_user_name);
            let r = GenTable::insert(&tx, &table).await?;
            if r.rows_affected > 0 {
                // 保存列信息
                let gen_table_columns = select_db_table_columns_by_name(&tx, &table_name).await?;
                for mut column in gen_table_columns {
                    column.column_id = Some(ObjectId::new().to_string());
                    column.create_by = oper_user_name.to_string().into();
                    column.create_time = crate::Now!().into();
                    gen_utils::init_column_field(&mut column, &table);
                    GenTableColumn::insert(&tx, &column).await?;
                }
            }
        }
        Ok(table_name_list.len() as u64)
    }

    pub async fn generate_code(&self, table_names: Vec<&str>) -> Result<()> {
        let tables =GenTable::select_by_map(pool!(),rbs::value!{"table_name":table_names}).await?;
        for t in tables {
            let code_map = self.generate(&t).await?;
            for (path, v) in code_map {
                fs::create_dir_all(&path.parent().unwrap())?;
                fs::write(path, v)?;
            }
        }
        Ok(())
    }
    pub async fn preview_code(&self, table_id: &str) -> Result<HashMap<String, String>> {
        let table = GenTable::select_by_map(pool!(), rbs::value!{"table_id": table_id})
            .await?
            .into_iter()
            .next();
        if let Some(t) = table {
            let code_map = self.generate(&t).await?;
            let mut res = HashMap::new();
            code_map.iter().for_each(|(path, v)| {
                let name = path.to_str().unwrap().to_string();
                let name = name.replace(&t.gen_path_back.clone().unwrap_or_default(), "");
                let name = name.replace(&t.gen_path_front.clone().unwrap_or_default(), "");
                res.insert(name, v.to_string());
            });
            Ok(res)
        } else {
            Err(Error::from("错误id"))
        }
    }

    async fn generate(&self, table: &GenTable) -> Result<HashMap<PathBuf, String>> {
        let mut code_map = HashMap::new();
        self.generate_code_of_lng(
            &table,
            &table.tpl_back_type.clone().unwrap_or_default(),
            &table.gen_path_back.clone().unwrap_or_default(),
            &mut code_map,
        )
        .await?;
        self.generate_code_of_lng(
            &table,
            &table.tpl_web_type.clone().unwrap_or_default(),
            &table.gen_path_front.clone().unwrap_or_default(),
            &mut code_map,
        )
        .await?;
        Ok(code_map)
    }
    /*
    生成各类语言
        */
    async fn generate_code_of_lng(
        &self,
        gen_table: &GenTable,
        language: &str,
        gen_path: &str,
        code_map: &mut HashMap<PathBuf, String>,
    ) -> Result<()> {
        let tlt_path = std::env::current_dir()?.join("template").join(language);
        use minijinja::Environment;
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                // .block_delimiters("{%", "%}")
                .variable_delimiters("${{", "}}")
                .comment_delimiters("{#", "#}")
                .build()
                .map_err(|e| Error::from(e.to_string()))?,
        );
        let tpl_path_list = find_files_with_extension(tlt_path.as_path(), "jinja")?;
        let mut tpl_name_list = vec![];
        tpl_path_list.into_iter().for_each(|path| {
            let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
            if !file_name.ends_with(".snap.jinja") {
                tpl_name_list.push(file_name.to_string());
                let contents = fs::read(&path)
                    .ok()
                    .map(|bs| String::from_utf8(bs).unwrap_or_default())
                    .unwrap_or_default();
                let _ = env.add_template_owned(file_name.to_string(), contents);
            }
        });

        let columns = GEN_CONTEXT
            .gen_table_column_service
            .select_gen_table_column_list_by_table_id(&gen_table.table_id.clone().unwrap_or_default())
            .await?;

        let table_name = gen_table.table_name.clone().unwrap_or_default();
        let module_name = gen_table.module_name.clone().unwrap_or_default();
        let ctx = jinja_utils::prepare_context(gen_table.clone(), columns);
        for tpl_name in &tpl_name_list {
            let file_name_render = env.render_str(tpl_name, &ctx).map_err(|e| Error::from(e.to_string()))?;
            let code = env
                .get_template(&tpl_name)
                .map_err(|e| Error::from(e.to_string()))?
                .render(&ctx)
                .map_err(|e| Error::from(e.to_string()))?;
            let mut file_name_split = file_name_render.split('.').collect::<Vec<&str>>();
            file_name_split.pop();
            let mut is_first = false;
            let mut suffix = file_name_split.pop().unwrap_or_default();
            if suffix.eq("first") {
                is_first = true;
                suffix = file_name_split.pop().unwrap_or_default();
            }
            let file_name = file_name_split.pop().unwrap_or_default();

            let path = PathBuf::from(&gen_path)
                .join(file_name_split.join("/"))
                .join(format!("{file_name}.{suffix}"));
            let mut code = code;

            let ext = path.extension().unwrap_or_default();
            if ext.eq("rs") {
                code = self.format_rust_code(&code)?;
            } else if ext.eq("vue") {
                code = self.format_html_code(&code, "vue")?;
            } else if ext.eq("js") {
                code = self.format_html_code(&code, "typescript")?;
            }
            loop {
                let t_code = code.replace("\n  \n", "\n").replace("\n\n", "\n");
                if t_code.len() == code.len() {
                    break;
                }
                code = t_code;
            }

            if is_first && path.exists() {
                println!("文件{}已存在！", path.display());
            } else {
                code_map.insert(path.clone(), code);
                // fs::write(&path, code.as_bytes())?;
            }
        }
        if language.eq("rust") {
            let path = PathBuf::from(&gen_path).join(&module_name);
            self.fill_mod(path.as_path(), true, code_map)?;
            self.fill_mod_in_module(path.join("mod.rs").as_path(), &table_name, code_map)?;
        }
        Ok(())
    }

    //完成mod.rs引用
    fn fill_mod(&self, dir: &Path, first_lvl: bool, code_map: &mut HashMap<PathBuf, String>) -> Result<()> {
        let mut file_names = Vec::new();
        if dir.is_dir() {
            let parent_name = dir.file_name().unwrap().to_str().unwrap().to_string();
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let idx = file_name.rfind(".").unwrap_or(file_name.len());
                let file_name = substring_unicode(file_name.as_str(), 0, idx);
                if path.is_dir() {
                    // 递归处理子目录
                    self.fill_mod(&path, false, code_map)?;
                    file_names.push(file_name);
                } else if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        if !file_name.eq("mod") {
                            file_names.push(file_name);
                        }
                    }
                }
            }
            let pub_mod = file_names
                .into_iter()
                .map(|s| {
                    if gen_constants::PUB_USE_NAME.contains(&parent_name.as_str()) {
                        format!("pub mod {s};\npub use {s}::*;")
                    } else {
                        format!("pub mod {s};")
                    }
                })
                .collect::<Vec<String>>();
            let path = dir.join("mod.rs");
            if first_lvl {
                if !fs::exists(dir.join("mod.rs"))? {
                    code_map.insert(path, pub_mod.join("\n"));
                    // fs::write(dir.join("mod.rs"), pub_mod.join("\n").as_bytes())?;
                }
            } else {
                code_map.insert(path, pub_mod.join("\n"));
                // fs::write(path, pub_mod.join("\n").as_bytes())?;
            }
        }

        Ok(())
    }

    //在src/modules/**/mod.rs插入数据，由temp.json填入
    fn fill_mod_in_module(
        &self,
        mod_rs_file: &Path,
        table_name: &str,
        code_map: &mut HashMap<PathBuf, String>,
    ) -> Result<()> {
        if !mod_rs_file.exists() {
            return Ok(());
        }
        let mut json = None;
        code_map.iter().for_each(|(k, v)| {
            if k.ends_with("temp.json") {
                json = Some(v.to_string());
            }
        });
       // let json_file = mod_rs_file.parent().unwrap().join("temp.json");
        if json.is_none() {
            return Ok(());
        }

        // let json = fs::read_to_string(json_file.as_path())?;
        //  fs::remove_file(json_file.as_path())?;
        let value: Value = serde_json::from_str(&json.unwrap_or_default()).map_err(|e| Error::from(e.to_string()))?;
        if let Value::Object(map) = value {
            for (k, v) in map {
                if let Value::String(s) = v {
                    let mod_rs = &fs::read_to_string(mod_rs_file)?;
                    let key = format!("//{}", k);
                    let end_key = format!("//end{}", k);
                    let auto_gen_key = format!("autogen_{}", table_name);
                    let idx = mod_rs.find(&key);
                    let idx_end = mod_rs.find(&end_key);
                    let mut is_have = false;
                    if idx.is_some() && idx_end.is_some() {
                        let substring =
                            substring_unicode(&mod_rs, idx.unwrap_or_default(), idx_end.unwrap_or_default());
                        let lines = substring.split("\r\n").collect::<Vec<&str>>();
                        for line in lines {
                            if line.find(&auto_gen_key).is_some() {
                                is_have = true;
                                break;
                            }
                        }
                    }
                    if !is_have {
                        let mod_rs = mod_rs.replace(&end_key, &format!("//{auto_gen_key}\n{s}\n{end_key}"));
                        code_map.insert(mod_rs_file.to_path_buf(), mod_rs);
                        //fs::write(mod_rs_file, mod_rs)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn format_rust_code(&self, code: &str) -> Result<String> {
        // 启动 rustfmt 进程
        let mut child = Command::new("rustfmt")
            .arg("--edition=2024")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // 将代码写入 rustfmt 的 stdin
        if let Some(stdin) = &mut child.stdin {
            stdin.write_all(code.as_bytes())?;
        }

        // 读取 rustfmt 的输出
        let output = child.wait_with_output()?;
        Ok(String::from_utf8(output.stdout).unwrap_or(code.to_string()))
    }

    pub fn format_html_code(&self, code: &str, language: &str) -> Result<String> {
        let mut child = Command::new("prettier.cmd")
            .arg("--print-width=160")
            .arg(format!("--parser={}", language))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = &mut child.stdin {
            stdin.write_all(code.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        Ok(String::from_utf8(output.stdout).unwrap_or(code.to_string()))
    }

    pub async fn synch_db(&self, table_name: &str) -> Result<()> {
        let table = GenTable::select_by_map(pool!(), rbs::value!{"table_name":table_name})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} ", table_name)))?;
        let old_table_columns = GEN_CONTEXT
            .gen_table_column_service
            .select_gen_table_column_list_by_table_id(&table.table_id.clone().unwrap_or_default())
            .await?;
        let mut table_column_map = HashMap::with_capacity(old_table_columns.len());

        old_table_columns.iter().for_each(|c| {
            table_column_map.insert(c.column_name.clone().unwrap_or_default(), c);
        });

        let db_table_columns = select_db_table_columns_by_name(pool!(), table_name).await?;

        if db_table_columns.len() == 0 {
            return Err(Error::from("同步数据失败，原表结构不存在"));
        }
        let db_table_column_names = db_table_columns
            .iter()
            .map(|c| c.column_name.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        for mut column in db_table_columns {
            gen_utils::init_column_field(&mut column, &table);
            let column_name = column.column_name.clone().unwrap_or_default();
            if let Some(prev_column) = table_column_map.get(&column_name) {
                column.column_id = prev_column.column_id.clone();

                if column.is_list.is_some_and(|c| c == gen_constants::REQUIRE) {
                    // 如果是列表，继续保留查询方式/字典类型选项
                    column.dict_type = prev_column.dict_type.clone();
                    column.query_type = prev_column.query_type.clone();
                }
                column.is_edit = prev_column.is_edit.clone();
                column.is_insert = prev_column.is_insert.clone();
                column.is_detail = prev_column.is_detail.clone();
                column.is_export = prev_column.is_export.clone();
                column.more = prev_column.more.clone();
                GenTableColumn::update_by_map(pool!(), &column, rbs::value!{"column_id":column.column_id.clone()}).await?;
            } else {
                column.column_id = Some(ObjectId::new().to_string());
                GenTableColumn::insert(pool!(), &column).await?;
            }
        }
        for column in old_table_columns {
            if !db_table_column_names.contains(&column.column_name.clone().unwrap_or_default()) {
                GenTableColumn::delete_by_map(pool!(), rbs::value!{"column_id":column.column_id}).await?;
            }
        }
        Ok(())
    }
}
