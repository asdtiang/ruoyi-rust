use  crate::system::domain::mapper::sys_config::SysConfig;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive( macros::Export)]
pub struct SysConfigVO {
    /** 参数主键 */
    #[excel( "参数主键")]
    pub config_id: Option<String>,
    /** 参数名称 */
    #[excel( "参数名称")]
    pub config_name: Option<String>,
    /** 参数键名 */
    #[excel( "参数键名")]
    pub config_key: Option<String>,
    /** 参数键值 */
    #[excel( "参数键值")]
    pub config_value: Option<String>,
    /** 系统内置（Y是 N否） */
    #[excel( "系统内置", readConverterExp = "Y=是,N=否")]
    pub config_type: Option<char>,
    pub remark: Option<String>,
}

impl From<SysConfig> for SysConfigVO {
    fn from(arg: SysConfig) -> Self {
        Self {
            config_id: arg.config_id,
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            remark: arg.remark,
        }
    }
}
