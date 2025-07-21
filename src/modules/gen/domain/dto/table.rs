use crate::gen::domain::mapper::gen_table::GenTable;
use crate::gen::domain::mapper::gen_table_column::GenTableColumn;

#[derive(serde :: Serialize, serde :: Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableNamesDTO {
    pub tables: Option<String>,
}

#[derive(serde :: Serialize, serde :: Deserialize, validator :: Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenTableUpdateDTO {
    pub table_id: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "表描述不能为空"))]
    pub table_comment: Option<String>,
    pub sub_table_name: Option<String>,
    pub sub_table_fk_name: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "实体类名称不能为空"))]
    pub class_name: Option<String>,
    pub tpl_category: Option<String>,
    pub tpl_web_type: Option<String>,pub tpl_back_type: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "生成包路径不能为空"))]
    pub package_name: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "生成模块名不能为空"))]
    pub module_name: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "生成业务名不能为空"))]
    pub business_name: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "生成功能名不能为空"))]
    pub function_name: Option<String>,
    #[validate(custom(function = "crate::string_required", message = "作者不能为空"))]
    pub function_author: Option<String>,
    pub fixed_header: Option<char>,
    pub gen_type: Option<String>,
    pub gen_path_back: Option<String>,pub gen_path_front: Option<String>,
    pub options: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub columns: Option<Vec<GenTableColumnUpdateDTO>>,
}
impl From<GenTableUpdateDTO> for GenTable {
    fn from(arg: GenTableUpdateDTO) -> Self {
        GenTable {
            table_id: arg.table_id,
            table_name: None,
            table_comment: arg.table_comment,
            sub_table_name: arg.sub_table_name,
            sub_table_fk_name: arg.sub_table_fk_name,
            class_name: arg.class_name,
            tpl_category: arg.tpl_category,
            tpl_web_type: arg.tpl_web_type,
            tpl_back_type: arg.tpl_back_type,
            package_name: arg.package_name,
            module_name: arg.module_name,
            business_name: arg.business_name,
            function_name: arg.function_name,
            function_author: arg.function_author,
            fixed_header: arg.fixed_header,
            gen_type: arg.gen_type,
            gen_path_back: arg.gen_path_back,
            gen_path_front: arg.gen_path_front,
            options: arg.options,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenTableColumnUpdateDTO {
    pub column_id: Option<String>,
    pub table_id: Option<String>,
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
    pub remark: Option<String>,
}
impl From<GenTableColumnUpdateDTO> for GenTableColumn {
    fn from(arg: GenTableColumnUpdateDTO) -> Self {
        let GenTableColumnUpdateDTO {
            column_id,
            table_id,
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
            remark,
        } = arg;

        Self {
            column_id,
            table_id,
            column_name: None,
            column_comment: None,
            column_type: None,
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
            dict_type:if dict_type.is_none(){Some("".to_string())} else{dict_type},
            sort,
            more,
            def_val,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark,
        }
    }
}
