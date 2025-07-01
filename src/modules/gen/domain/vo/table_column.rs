use super::super::mapper::gen_table_column::GenTableColumn;
use crate::gen::service::gen_constants;
use crate::utils::string::substring;
use convert_case::{Case, Casing};
use rbatis::rbdc::DateTime;

#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenTableColumnVO {
    pub column_id: Option<String>,
    pub table_id: Option<String>,
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    pub column_type: Option<String>,
    pub java_type: Option<String>,
    pub java_field: Option<String>,
    pub is_pk: Option<char>,
    pub is_increment: Option<char>,
    pub is_required: Option<char>,
    pub is_insert: Option<char>,
    pub is_edit: Option<char>,
    pub is_list: Option<char>,
    pub is_detail: Option<char>,
    pub is_export: Option<char>,
    pub is_sortable: Option<char>,
    pub is_query: Option<char>,
    pub query_type: Option<String>,
    pub html_type: Option<String>,
    pub dict_type: Option<String>,
    pub sort: Option<u32>,
    pub more: Option<serde_json::Value>,
    pub def_val: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    #[serde(with = "crate::utils::date_time_format")]
    pub create_time: Option<DateTime>,
    #[serde(with = "crate::utils::date_time_format")]
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
impl From<GenTableColumn> for GenTableColumnVO {
    fn from(arg: GenTableColumn) -> Self {
        let GenTableColumn {
            column_id,
            table_id,
            column_name,
            column_comment,
            column_type,
            java_type,
            java_field,
            is_pk,
            is_increment,
            is_required,
            is_insert,
            is_edit,
            is_list,
            is_detail,
            is_export,
            is_sortable,
            is_query,
            query_type,
            html_type,
            dict_type,
            sort,
            more,
            def_val,
            create_by,
            create_time,
            update_by,
            update_time,
            remark,
        } = arg;

        Self {
            column_id,
            table_id,
            column_name,
            column_comment,
            column_type,
            java_type,
            java_field,
            is_pk,
            is_increment,
            is_required,
            is_insert,
            is_edit,
            is_list,
            is_detail,
            is_export,
            is_sortable,
            is_query,
            query_type,
            html_type,
            dict_type,
            sort,
            more,
            def_val,
            create_by,
            create_time,
            update_by,
            update_time,
            remark,
        }
    }
}

#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenTableColumnGenVO {
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    pub comment: Option<String>,
    pub column_type: Option<String>,
    pub read_converter_exp: Option<String>,
    pub java_type: Option<String>,
    pub java_field: Option<String>,
    pub java_field_cap: Option<String>,
    pub is_pk: bool,
    pub is_increment: bool,
    pub is_required: bool,
    pub is_insert: bool,
    pub is_edit: bool,
    pub is_list: bool,
    pub is_detail: bool,
    pub is_export: bool,
    pub is_sortable: bool,
    pub is_query: bool,
    pub query_type: Option<String>,
    pub html_type: Option<String>,
    pub dict_type: Option<String>,
    pub sort: Option<u32>,
    pub more: Option<serde_json::Value>,
    pub max_length: Option<usize>,
    pub precision: Option<usize>,
    pub def_val: Option<String>,
    //特殊字段，即是："id", "create_by", "create_time", "del_flag", "update_by",
    // "update_time"
    pub special:bool,
    pub max: Option<usize>,
    pub min: Option<usize>,
}
impl From<GenTableColumn> for GenTableColumnGenVO {
    fn from(arg: GenTableColumn) -> Self {
        let GenTableColumn {
            column_name,
            column_comment,
            column_type,
            java_type,
            java_field,
            is_pk,
            is_increment,
            is_required,
            is_insert,
            is_edit,
            is_list,
            is_detail,
            is_export,
            is_sortable,
            is_query,
            query_type,
            html_type,
            dict_type,
            sort,
            more,
            def_val,
            ..
        } = arg;
        let mut read_converter_exp =None;
        let comment = column_comment.clone();
        let comment = comment.map(|s| {
            let mut idx = s.find("(");
            if idx.is_none() {
                idx = s.find("（");
            }
            match idx {
                None => s,
                Some(idx) => {
                    read_converter_exp=Some(substring(&s,idx+1,s.len()-1));
                    substring(&s, 0, idx) },
            }
        });
        let java_field_cap = java_field.clone().map(|s| s.to_case(Case::UpperCamel));
        Self {
            column_name:column_name.clone(),
            column_comment,
            comment,
            read_converter_exp,
            column_type,
            java_type,
            java_field,
            java_field_cap,
            is_pk: is_pk.is_some_and(|b| b == gen_constants::REQUIRE),
            is_increment: is_increment.is_some_and(|b| b == gen_constants::REQUIRE),
            is_required: is_required.is_some_and(|b| b == gen_constants::REQUIRE),
            is_insert: is_insert.is_some_and(|b| b == gen_constants::REQUIRE),
            is_edit: is_edit.is_some_and(|b| b == gen_constants::REQUIRE),
            is_list: is_list.is_some_and(|b| b == gen_constants::REQUIRE),
            is_detail: is_detail.is_some_and(|b| b == gen_constants::REQUIRE),
            is_export: is_export.is_some_and(|b| b == gen_constants::REQUIRE),
            is_sortable: is_sortable.is_some_and(|b| b == gen_constants::REQUIRE),
            is_query: is_query.is_some_and(|b| b == gen_constants::REQUIRE),
            query_type,
            html_type,
            dict_type,
            sort,
            more,
            max_length: None,
            precision: None,
            def_val,
            special: gen_constants::COLUMNNAME_NOT_LIST.contains(&column_name.unwrap_or_default().as_str()),
            max: None,
            min: None,
        }
    }
}
