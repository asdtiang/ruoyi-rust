use macros::Export;
use  crate::system::domain::mapper::sys_dict_type::SysDictType;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Export)]
pub struct SysDictTypeVO {
    /** 字典主键 */
    #[excel( "字典主键")]
    pub dict_id: Option<String>,
    /** 字典名称 */
    #[excel( "字典名称")]
    pub dict_name: Option<String>,
    /** 字典类型 */
    #[excel( "字典类型")]
    pub dict_type: Option<String>,
    /** 状态（0正常 1停用） */
    #[excel( "状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<SysDictType> for SysDictTypeVO {
    fn from(arg: SysDictType) -> Self {
        Self {
            dict_id: arg.dict_id,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}

impl SysDictTypeVO {}
