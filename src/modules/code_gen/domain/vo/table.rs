use super::super::mapper::gen_table::GenTable;
use rbatis::rbdc::DateTime;

#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenTableVO {
    pub table_id: Option<String>,
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
    // pub sub_table_name: Option<String>,
    // pub sub_table_fk_name: Option<String>,
    pub struct_name: Option<String>,
    pub tpl_category: Option<String>,
    pub tpl_web_type: Option<String>,
    pub tpl_back_type: Option<String>,
    pub package_name: Option<String>,
    pub module_name: Option<String>,
    pub business_name: Option<String>,
    pub function_name: Option<String>,
    pub function_author: Option<String>,
    pub switch_opt: Option<serde_json::Value>,
    pub gen_type: Option<String>,
    pub gen_path_back: Option<String>,pub gen_path_web: Option<String>,
    pub options: Option<serde_json::Value>,
    #[serde(with = "crate::utils::date_time_format")]
    pub create_time: Option<DateTime>,
    #[serde(with = "crate::utils::date_time_format")]
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
impl From<GenTable> for GenTableVO {
    fn from(arg: GenTable) -> Self {
        Self {
            table_id: arg.table_id,
            table_name: arg.table_name,
            table_comment: arg.table_comment,
            // sub_table_name: arg.sub_table_name,
            // sub_table_fk_name: arg.sub_table_fk_name,
            struct_name: arg.struct_name,
            tpl_category: arg.tpl_category,
            tpl_web_type: arg.tpl_web_type,
            tpl_back_type:  arg.tpl_back_type,
            package_name: arg.package_name,
            module_name: arg.module_name,
            business_name: arg.business_name,
            function_name: arg.function_name,
            function_author: arg.function_author,
            switch_opt: arg.switch_opt,
            gen_type: arg.gen_type,
            gen_path_back:  arg.gen_path_back,
            gen_path_web:  arg.gen_path_web,
            options: arg.options,
            create_time: arg.create_time,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}