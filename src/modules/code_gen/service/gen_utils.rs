use crate::code_gen::domain::mapper::gen_table::GenTable;
use crate::code_gen::domain::mapper::gen_table_column::GenTableColumn;
use crate::code_gen::service::gen_constants;
use crate::code_gen::GEN_CONTEXT;
use crate::utils::string;
use crate::utils::string::{substring, substring_between};
use convert_case::{Case, Casing};
use serde_json::json;
use std::collections::HashMap;

/**
 * 代码生成器 工具类
 *
 * @author ruoyi
 */

/**
 * 初始化表信息
 */
pub fn init_table(gen_table: &mut GenTable, oper_name: &str) {
    gen_table.struct_name = convert_struct_name(&gen_table.table_name.clone().unwrap_or_default()).into();
    gen_table.package_name = GEN_CONTEXT.config.package_name.clone().into();
    let table_name = gen_table.table_name.clone().unwrap_or_default();
    let idx = table_name.find('_');
    if idx.is_some_and(|u| u > 0) {
        let (module_name,business_name) = table_name.split_once("_").unwrap_or_default();
        gen_table.module_name = module_name.to_string().into();
        gen_table.business_name = business_name.to_case(Case::Camel).into();
    } else {
        gen_table.module_name = get_module_name(&GEN_CONTEXT.config.package_name.clone()).into();
        gen_table.business_name = gen_table.table_name.clone();
    }

    gen_table.function_name = replace_text(&gen_table.table_comment.clone().unwrap_or_default()).into();
    gen_table.function_author = GEN_CONTEXT.config.author.clone().into();
    gen_table.tpl_back_type = Some("rust".to_string());
    gen_table.tpl_web_type = Some("element-plus".to_string());
    gen_table.create_by = oper_name.to_string().into();
    gen_table.create_time = crate::Now!().into();
}

/**
 * 初始化列属性字段
 */
pub fn init_column_field(column: &mut GenTableColumn, table: &GenTable) {
    let binding = column.column_type.clone().unwrap_or_default();
    let column_type = binding.as_str();
    let binding2 = column.column_name.clone().unwrap_or_default();
    let column_name = binding2.as_str();
    let binding3 = get_db_type(&column.column_type.clone().unwrap_or_default());
    let data_type = binding3.as_str();
    println!("data_type {}", data_type);

    column.table_id = table.table_id.clone();
    column.create_by = table.create_by.clone();
    // 设置rust字段名
    column.rust_field = column_name.to_string().into();
    // 设置默认类型
    column.rust_type = Some(gen_constants::TYPE_STRING.to_string());
    column.query_type = Some(gen_constants::QUERY_EQ.to_string());

    column.html_type = Some(gen_constants::HTML_INPUT.to_string());
    column.is_insert = Some(gen_constants::REQUIRE);
    column.is_edit = Some(gen_constants::REQUIRE);
    column.is_list = Some(gen_constants::REQUIRE);
    column.is_detail = Some(gen_constants::REQUIRE);
    column.is_query = Some(gen_constants::REQUIRE);
    column.is_export = Some(gen_constants::REQUIRE);

    let mut more = HashMap::new();

    if gen_constants::COLUMNTYPE_STR.contains(&data_type) || gen_constants::COLUMNTYPE_TEXT.contains(&data_type) {
        let column_length = get_column_length(column_type);
        if column_length == 1 && data_type.eq("char") {
            column.html_type = Some(gen_constants::HTML_INPUT.to_string());
            column.rust_type = Some(gen_constants::TYPE_CHAR.to_string());
        } else {
            // 字符串长度超过200设置为文本域
            let html_type = if column_length >= 200 || gen_constants::COLUMNTYPE_TEXT.contains(&data_type) {
                column.is_list = Some(gen_constants::NOT_REQUIRE);
                column.is_query = Some(gen_constants::NOT_REQUIRE);
                column.query_type = Some(gen_constants::QUERY_LIKE.to_string());
                gen_constants::HTML_TEXTAREA
            } else {
                gen_constants::HTML_INPUT
            };
            column.html_type = Some(html_type.to_string());
            more.insert("checkLength".to_string(), "1".to_string());
            column.more = Some(json!(more));
        }
    } else if gen_constants::COLUMNTYPE_TIME.contains(&data_type) {
        column.rust_type = Some(gen_constants::TYPE_DATE.to_string());
        column.html_type = Some(gen_constants::HTML_DATE.to_string());
        column.query_type=Some("BETWEEN".to_string());
        if "datetime".eq(data_type) || "timestamp".eq(data_type) {
            column.rust_type = Some(gen_constants::TYPE_TIMESTAMP.to_string());
            column.html_type = Some(gen_constants::HTML_DATETIME.to_string());
        } else if "time".eq(data_type) {
            column.rust_type = Some(gen_constants::TYPE_TIME.to_string());
            column.html_type = Some(gen_constants::HTML_TIME.to_string());
        }
    } else if gen_constants::COLUMNTYPE_NUMBER.contains(&data_type) {
        column.query_type=Some("BETWEEN".to_string());
        column.html_type = Some(gen_constants::HTML_NUMBER.to_string());
        // 如果是浮点型 统一用BigDecimal
        let str = string::substring_between(column_type, "(", ")");
        let nums = str.split(",").map(|s|s.parse::<usize>().unwrap_or_default()).collect::<Vec<_>>();
        if nums.len() == 2 && nums[1] > 0 {
            column.rust_type = Some(gen_constants::TYPE_BIGDECIMAL.to_string());
        }
        // 如果是整形
        else if nums.len() == 1 && data_type.eq("bigint") {
            column.rust_type = Some(gen_constants::TYPE_LONG.to_string());
        }
        else if nums.len() == 1 && nums[0]==1 && (data_type.eq("bit")||data_type.eq("tinyint")) {
            column.rust_type = Some(gen_constants::TYPE_BOOLEAN.to_string());
        }
        // 长整形
        else {
            column.rust_type = Some(gen_constants::TYPE_INTEGER.to_string());
        }
    } else if data_type.eq("json") {
        column.html_type = Some(gen_constants::HTML_INPUT.to_string());
        column.is_query = Some(gen_constants::NOT_REQUIRE);
        column.is_export = Some(gen_constants::NOT_REQUIRE);
        column.rust_type = Some(gen_constants::TYPE_OBJECT_JSON.to_string());
    }
    if column_name.starts_with("是否"){
        column.rust_type = Some(gen_constants::TYPE_BOOLEAN.to_string());
    }

    if column.rust_type.eq(&Some(gen_constants::TYPE_BOOLEAN.to_string())){
        column.query_type=Some("EQ".to_string());
        column.html_type = Some(gen_constants::HTML_RADIO.to_string());
    }

    let is_pk = column.is_pk.clone().unwrap_or_default() == '1';
    if gen_constants::COLUMNNAME_NOT_INSERT.contains(&column_name) {
        column.is_insert = Some(gen_constants::NOT_REQUIRE);
    }
    if gen_constants::COLUMNNAME_NOT_EDIT.contains(&column_name) {
        column.is_edit = Some(gen_constants::NOT_REQUIRE);
    }
    if gen_constants::COLUMNNAME_NOT_LIST.contains(&column_name) {
        column.is_list = Some(gen_constants::NOT_REQUIRE);
        column.is_detail = Some(gen_constants::NOT_REQUIRE);
        column.is_export = Some(gen_constants::NOT_REQUIRE);
    }
    if gen_constants::COLUMNNAME_NOT_QUERY.contains(&column_name) {
        column.is_query = Some(gen_constants::NOT_REQUIRE);
    }
    if is_pk {
        column.is_list = Some(gen_constants::NOT_REQUIRE);
        column.is_edit = Some(gen_constants::NOT_REQUIRE);
        column.is_detail = Some(gen_constants::NOT_REQUIRE);
        column.is_export = Some(gen_constants::NOT_REQUIRE);
        column.is_query = Some(gen_constants::NOT_REQUIRE);
    }

    // 查询字段
    if column.is_query.is_none() && !gen_constants::COLUMNNAME_NOT_QUERY.contains(&column_name) && !is_pk {
        column.is_query = Some(gen_constants::REQUIRE);
    }

    column.is_table=column.is_list.clone();
    // 查询字段类型
    let lowercase_column_name = column_name.to_lowercase();
    if lowercase_column_name.ends_with("name") {
        column.query_type = Some(gen_constants::QUERY_LIKE.to_string());
    }
    // 状态字段设置单选框
    if lowercase_column_name.ends_with("status") {
        column.html_type = Some(gen_constants::HTML_RADIO.to_string());
    }
    // 类型&性别字段设置下拉框
    else if lowercase_column_name.ends_with("type") || lowercase_column_name.ends_with("sex") {
        column.html_type = Some(gen_constants::HTML_SELECT.to_string());
    }
    // 图片字段设置图片上传控件
    else if lowercase_column_name.ends_with("image")
        || lowercase_column_name.ends_with("images")
        || lowercase_column_name.ends_with("picture")
        || lowercase_column_name.ends_with("pictures")
    {
        column.html_type = Some(gen_constants::HTML_IMAGE_UPLOAD.to_string());
        if data_type.eq("json") {
            column.rust_type = Some(gen_constants::TYPE_ARRAY_JSON.to_string());
        }
    }
    // 文件字段设置文件上传控件
    else if lowercase_column_name.ends_with("file") {
        column.html_type = Some(gen_constants::HTML_FILE_UPLOAD.to_string());
    }
    // 内容字段设置富文本控件
    else if lowercase_column_name.ends_with("content") {
        column.html_type = Some(gen_constants::HTML_EDITOR.to_string());
    }
}

/**
 * 获取模块名
 *
 * @param package_name 包名
 * @return 模块名
 */
pub fn get_module_name(package_name: &str) -> String {
    let last_index = package_name.rfind(".");
    match last_index {
        Some(index) => {
            let name_length = package_name.len();
            substring(package_name, index + 1, name_length)
        }
        None => package_name.to_string(),
    }
}


/**
 * 表名转换成Java类名
 *
 * @param table_name 表名称
 * @return 类名
 */
pub fn convert_struct_name(table_name: &str) -> String {
    let auto_remove_pre = GEN_CONTEXT.config.auto_remove_pre;
    let table_prefixes = GEN_CONTEXT.config.table_prefixes.clone();
    let mut new_table_name = table_name.to_string();
    if auto_remove_pre && !table_prefixes.is_empty() {
        new_table_name = replace_first(table_name, table_prefixes);
    }
    new_table_name.to_case(Case::UpperCamel)
}

/**
 * 批量替换前缀
 *
 * @param replacementm 替换值
 * @param search_list   替换列表
 * @return
 */
pub fn replace_first(replacementm: &str, search_list: Vec<String>) -> String {
    for search_string in search_list {
        if replacementm.starts_with(&search_string) {
            return replacementm.replacen(&search_string, "", 1).to_string();
        }
    }
    replacementm.to_string()
}

/**
 * 关键字替换
 *
 * @param text 需要被替换的名字
 * @return 替换后的名字
 */
pub fn replace_text(text: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"(?:表|若依)").unwrap();
    re.replace_all(text, "").to_string()
}

/**
 * 获取数据库类型字段
 *
 * @param column_type 列类型
 * @return 截取后的列类型
 */
pub fn get_db_type(column_type: &str) -> String {
    match column_type.find("(") {
        Some(index) => substring(column_type, 0, index),
        None => column_type.to_string(),
    }
}

/**
 * 获取字段长度
 *
 * @param column_type 列类型
 * @return 截取后的列类型
 */
pub fn get_column_length(column_type: &str) -> usize {
    let s=substring_between(column_type,"(",")");
    s.parse::<usize>().unwrap_or(0)
}
