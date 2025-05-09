use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select_page};

crud!(SysOperLog {});

impl_select_page!( SysOperLog{select_page(dto: &crate::system::domain::dto::OperLogPageDTO) =>
    "``
      if do_count == false:
         ` order by oper_time desc`"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysOperLog {
    /** 日志主键 */
    //  @Excel(name = "操作序号", cellType = ColumnType.NUMERIC)
    pub oper_id: Option<String>,

    /** 操作模块 */
    //  @Excel(name = "操作模块")
    pub title: Option<String>,

    /** 业务类型（0其它 1新增 2修改 3删除） */
    //  @Excel(name = "业务类型", readConverterExp = "0=其它,1=新增,2=修改,3=删除,4=授权,5=导出,6=导入,7=强退,8=生成代码,9=清空数据")
    pub business_type: Option<u16>,

    /** 业务类型数组 */
    //  pub Integer[] businessTypes:Option<String>,

    /** 请求方法 */
    //  @Excel(name = "请求方法")
    pub method: Option<String>,

    /** 请求方式 */
    //  @Excel(name = "请求方式")
    pub request_method: Option<String>,

    /** 操作类别（0其它 1后台用户 2手机端用户） */
    //  @Excel(name = "操作类别", readConverterExp = "0=其它,1=后台用户,2=手机端用户")
    pub operator_type: Option<u16>,

    /** 操作人员 */
    //  @Excel(name = "操作人员")
    pub oper_name: Option<String>,

    /** 部门名称 */
    //  @Excel(name = "部门名称")
    pub dept_name: Option<String>,

    /** 请求url */
    //  @Excel(name = "请求地址")
    pub oper_url: Option<String>,

    /** 操作地址 */
    //  @Excel(name = "操作地址")
    pub oper_ip: Option<String>,

    /** 操作地点 */
    //  @Excel(name = "操作地点")
    pub oper_location: Option<String>,

    /** 请求参数 */
    //  @Excel(name = "请求参数")
    pub oper_param: Option<String>,

    /** 返回参数 */
    //  @Excel(name = "返回参数")
    pub json_result: Option<String>,

    /** 操作状态（0正常 1异常） */
    //  @Excel(name = "状态", readConverterExp = "0=正常,1=异常")
    pub status: Option<char>,

    /** 错误消息 */
    //  @Excel(name = "错误消息")
    pub error_msg: Option<String>,

    /** 操作时间 */
    // @JsonFormat(pattern = "yyyy-MM-dd HH:mm:ss")
    //  @Excel(name = "操作时间", width = 30, dateFormat = "yyyy-MM-dd HH:mm:ss")
    pub oper_time: Option<DateTime>,

    /** 消耗时间 */
    //  @Excel(name = "消耗时间", suffix = "毫秒")
    pub cost_time: Option<u64>,
}
