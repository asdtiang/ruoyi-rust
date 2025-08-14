use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysOperLog {});

impl_select_page!( SysOperLog{select_page(dto: &crate::system::domain::dto::OperLogPageDTO) =>
    "order by oper_time desc"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysOperLog {
    /** 日志主键 */
    pub oper_id: Option<String>,

    /** 操作模块 */
    pub title: Option<String>,

    /** 业务类型（0其它 1新增 2修改 3删除） */
    pub business_type: Option<u16>,

    /** 业务类型数组 */
    //  pub Integer[] businessTypes:Option<String>,

    /** 请求方法 */
    pub method: Option<String>,

    /** 请求方式 */
    pub request_method: Option<String>,

    /** 操作类别（0其它 1后台用户 2手机端用户） */
    pub operator_type: Option<u16>,

    /** 操作人员 */
    pub oper_name: Option<String>,

    /** 部门名称 */
    pub dept_name: Option<String>,

    /** 请求url */
    pub oper_url: Option<String>,

    /** 操作地址 */
    //  @Excel(name = "操作地址")
    pub oper_ip: Option<String>,

    /** 操作地点 */
    pub oper_location: Option<String>,

    /** 请求参数 */
    pub oper_param: Option<String>,

    /** 返回参数 */
    pub json_result: Option<String>,

    /** 操作状态（0正常 1异常） */
    pub status: Option<char>,

    /** 错误消息 */
    pub error_msg: Option<String>,

    /** 操作时间 */
    pub oper_time: Option<DateTime>,

    /** 消耗时间 */
    pub cost_time: Option<u64>,
}
