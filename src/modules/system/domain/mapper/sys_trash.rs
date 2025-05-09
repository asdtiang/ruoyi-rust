use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_delete};
crud!(SysTrash {});
impl_delete!(SysTrash{ delete_by_day_before(before:DateTime) => "` where create_date < #{before}`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}