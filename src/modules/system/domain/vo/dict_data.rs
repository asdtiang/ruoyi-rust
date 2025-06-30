use macros::Export;
use crate::system::domain::mapper::sys_dict_data::SysDictData;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Export)]
pub struct SysDictDataVO {
    /** 字典编码 */
    #[excel( "字典编码")]
    pub dict_code: Option<String>,
    /** 字典排序 */
    #[excel( "字典排序")]
    pub dict_sort: Option<u32>,
    /** 字典标签 */
    #[excel( "字典标签")]
    pub dict_label: Option<String>,
    /** 字典键值 */
    #[excel( "字典键值")]
    pub dict_value: Option<String>,
    /** 字典类型 */
    #[excel( "字典类型")]
    pub dict_type: Option<String>,
    /** 样式属性（其他样式扩展） */
    pub css_class: Option<String>,
    /** 表格字典样式 */
    pub list_class: Option<String>,
    /** 是否默认（Y是 N否） */
    #[excel( "是否默认", readConverterExp = "Y=是,N=否")]
    pub is_default: Option<String>,
    /** 状态（0正常 1停用） */
    #[excel( "状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<SysDictData> for SysDictDataVO {
    fn from(arg: SysDictData) -> Self {
        Self {
            dict_code: arg.dict_code,
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}

impl SysDictDataVO {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDictDataSimpleVO {
    /** 字典标签 */
    pub dict_label: String,
    /** 字典键值 */
    pub dict_value: String,
    /** 样式属性（其他样式扩展） */
    pub css_class: String,
    /** 表格字典样式 */
    pub list_class: String,
}

impl From<SysDictData> for SysDictDataSimpleVO {
    fn from(arg: SysDictData) -> Self {
        Self {
            dict_label: arg.dict_label.unwrap_or_default(),
            dict_value: arg.dict_value.unwrap_or_default(),
            css_class: arg.css_class.unwrap_or_default(),
            list_class: arg.list_class.unwrap_or_default(),
        }
    }
}

impl SysDictDataSimpleVO {}
