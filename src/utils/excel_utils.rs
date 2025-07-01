use crate::system::service::dict_utils;
use rust_xlsxwriter::{ColNum, Color, Format, Workbook};
use serde::de::DeserializeOwned;
use serde::Serialize;
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
    pub attr_type: Option<AttrType>,
}

pub struct ExcelGen {
    pub camel_case_indent: String,
    pub name: String,
    pub default_value: Option<String>,
    pub dict_data_map: Option<HashMap<String, String>>,
    pub read_converter_map: Option<HashMap<String, String>>,
    pub num_format: Option<rust_xlsxwriter::Format>,
    pub width: Option<f64>,
    pub attr_type: AttrType,
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
        let mut dict_data_map = None;
        if let Some(dict_type) = attr.dict_type.clone() {
            let mut exp = HashMap::new();
            dict_type.split(",").for_each(|s| {
                let ss = s.split("=").collect::<Vec<&str>>();
                if ss.len() == 2 {
                    exp.insert(ss[0].to_string(), ss[1].to_string());
                }
            });
            dict_data_map = Some(exp);
        }
        let num_format = attr
            .num_format
            .map(|f| rust_xlsxwriter::Format::new().set_num_format(f));
        Self {
            camel_case_indent: attr.camel_case_indent,
            name: attr.name,
            default_value: attr.default_value,
            dict_data_map,
            read_converter_map,
            num_format,
            width: attr.width,
            attr_type: attr.attr_type.unwrap_or(AttrType::ALL),
        }
    }
}

pub trait ExcelGenAttrTrait {
    fn get_excel_attr() -> Vec<ExcelGenAttr>;
}
/**
 * 字段类型（0：导出导入；1：仅导出；2：仅导入）
 */
#[derive(Debug, Clone)]
pub enum AttrType {
    ALL,
    EXPORT,
    IMPORT,
}

pub async fn to_excel<T>(res: &Vec<T>) -> crate::error::Result<Vec<u8>>
where
    T: ExcelGenAttrTrait + Serialize + DeserializeOwned + Clone,
{
    let mut excel_gens = Vec::new();
    for excel_attr in T::get_excel_attr() {
        if excel_attr.attr_type.clone().is_some_and(|a| {
            return match a {
                AttrType::IMPORT => true,
                _ => false,
            };
        }) {
            continue;
        }
        let dict_type = excel_attr.dict_type.clone();
        let mut excel_gen = ExcelGen::from(excel_attr);
        if let Some(dict_type) = dict_type {
            let dict_data_map = dict_utils::get_dict_label_map(&dict_type).await?;
            excel_gen.dict_data_map = Some(dict_data_map);
        }
        excel_gens.push(excel_gen);
    }

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold_format = Format::new().set_bold().set_background_color(Color::Gray);
    for (i, excel_gen) in excel_gens.iter_mut().enumerate() {
        // Write headers
        worksheet.write_string_with_format(0, i as ColNum, excel_gen.name.clone(), &bold_format)?;
        worksheet.set_column_width(i as ColNum, excel_gen.width.unwrap_or(16.0))?;
    }
    // Write data
    for (row, vo) in res.iter().enumerate() {
        let row = row as u32 + 1;
        let values = serde_json::json!(vo);
        for (col, excel_gen) in excel_gens.iter().enumerate() {
            let value = match values.get(&excel_gen.camel_case_indent) {
                None => &excel_gen.default_value.clone().unwrap_or_default(),
                Some(e) => {
                    if e.is_number() {
                        let v = e.as_f64().map(|n| n as f64).unwrap_or(0.0);
                        worksheet.write_number(row, col as ColNum, v)?;
                        if let Some(num_format) = excel_gen.num_format.clone() {
                            worksheet.set_cell_format(row, col as ColNum, &num_format)?;
                        }
                    }
                    e.as_str().unwrap_or_default()
                }
            };
            //只处理string
            if value.len() > 0 {
                let to_write = if let Some(dict_data_map) = excel_gen.dict_data_map.clone() {
                    &(dict_data_map
                        .get(value)
                        .map(|s| s.clone())
                        .unwrap_or_default()
                        .clone())
                } else if let Some(map) = excel_gen.read_converter_map.clone() {
                    &(map
                        .get(value)
                        .map(|s| s.clone())
                        .unwrap_or_default()
                        .clone())
                } else {
                    value
                };
                worksheet.write_string(row, col as ColNum, to_write)?;
            }
        }
    }

    Ok(workbook.save_to_buffer()?)
}
