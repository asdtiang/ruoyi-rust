use  crate::system::domain::mapper::sys_dict_type::SysDictType;
use macros::page_request;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;

use serde::{Deserialize, Serialize};

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageDTO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeAddDTO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub remark: Option<String>
}

impl From<DictTypeAddDTO> for SysDictType {
    fn from(arg: DictTypeAddDTO) -> Self {
        SysDictType {
            dict_id: ObjectId::new().to_string().into(),
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: Some(crate::web_data::get_user_name()),
            create_time: DateTime::now().set_nano(0).into(),
            update_by: None,
            update_time: None,
            remark: arg.remark
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeUpdateDTO {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<char>,
    pub remark: Option<String>,
}

impl From<DictTypeUpdateDTO> for SysDictType {
    fn from(arg: DictTypeUpdateDTO) -> Self {
        SysDictType {
            dict_id: arg.dict_id,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            status: arg.status,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: DateTime::now().set_nano(0).into(),
            remark: arg.remark,
        }
    }
}
