use std::collections::HashMap;

#[derive(Debug)]
pub struct ExcelGenAttr {
    pub camel_case_indent: String,
    pub name: String,
    pub dict_type: Option<String>,
    pub default_value: Option<String>,
    pub read_converter_exp: Option<String>,
    pub num_format: Option<String>,
    pub width: Option<f64>,
}
pub struct ExcelGen {
    pub camel_case_indent: String,
    pub name: String,
    pub default_value: Option<String>,
    pub dict_data_map: Option<HashMap<String, String>>,
    pub read_converter_map: Option<HashMap<String, String>>,
}
impl From<ExcelGenAttr> for ExcelGen {
    fn from(attr: ExcelGenAttr) -> Self {
        let mut read_converter_map = None;
        if let Some(read_converter_exp) = attr.read_converter_exp.clone() {
            let mut exp = HashMap::new();
            read_converter_exp.split(",").for_each(|s| {
                let ss = s.split("=").collect::<Vec<&str>>();
                if ss.len() == 2 {
                    exp.insert(ss[0].to_string(), ss[1].to_string());
                }
            });
            read_converter_map = Some(exp);
        }
        Self {
            camel_case_indent: attr.camel_case_indent,
            name: attr.name,
            default_value: attr.default_value,
            dict_data_map: None,
            read_converter_map,
        }
    }
}
/**
 * 字段类型（0：导出导入；1：仅导出；2：仅导入）
 */
pub enum AttrType {
    ALL,
    EXPORT,
    IMPORT,
}
