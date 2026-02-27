use macros::page_request;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql, impl_select_page, pysql_select_page};
use rbs::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenTable {
    //编号
    pub table_id: Option<String>,
    //表名称
    pub table_name: Option<String>,
    //表描述
    pub table_comment: Option<String>,
    // //子表或者连接表表名
    // pub sub_table_name: Option<String>,
    // //子表或者关联表关联父表的外键名
    // pub sub_table_fk_name: Option<String>,
    //实体类名称(首字母大写)
    pub struct_name: Option<String>,
    //使用的模板（crud单表操作 tree树表操作 sub主子表操作）
    pub tpl_category: Option<String>,
    //前端类型（element-ui模版 element-plus模版）
    pub tpl_web_type: Option<String>,
    pub tpl_back_type: Option<String>,
    //生成包路径
    pub package_name: Option<String>,
    //生成模块名
    pub module_name: Option<String>,
    //生成业务名
    pub business_name: Option<String>,
    //生成功能名
    pub function_name: Option<String>,
    //生成作者
    pub function_author: Option<String>,
    //* vue固定表头
    pub switch_opt: Option<serde_json::Value>,
    //生成代码方式（0zip压缩包 1自定义路径）
    pub gen_type: Option<String>,
    //前端生成路径（不填默认项目路径）
    pub gen_path_back: Option<String>,
    //后台生成路径（不填默认项目路径）
    pub gen_path_web: Option<String>,
    //表列信息
    pub options: Option<serde_json::Value>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
crud!(GenTable {});

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TablePageDTO {
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
}

//fixme 未修正
impl_select_page!(GenTable{select_page(dto: &TablePageDTO) =>
"trim start=' where ':
    ` where `
    if dto.configName != '':
        ` and config_name like #{'%'+dto.configName+'%'}`
    if dto.configKey != '':
        ` and config_key like #{'%'+dto.configKey+'%'}`
    if dto.configType != '':
        ` and config_type = #{dto.configType}`
    if dto.status != '':
        ` and status = #{dto.status}`
    if dto.params.beginTime != '':
        ` and date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
        ` and date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    if do_count == false:
     ` order by create_time`"});

pysql_select_page!(select_db_table_list(dto:&TablePageDTO) -> GenTable =>
    r#"select TABLE_NAME table_name, TABLE_COMMENT table_comment, CREATE_TIME create_time, UPDATE_TIME update_time from information_schema.tables
        ` where table_schema = (select database())`
        ` AND table_name NOT LIKE 'qrtz_%' AND table_name NOT LIKE 'gen_%'`
        ` AND table_name NOT IN (select table_name from gen_table)`
    if dto.tableName != '':
        ` AND lower(table_name) like lower(concat('%', #{dto.tableName}, '%'))`
    if dto.tableComment != '':
        ` AND lower(table_comment) like lower(concat('%', #{dto.tableComment}, '%'))`
    if dto.params.beginTime != '':
        ` AND date_format(create_time,'%y%m%d') >= date_format(#{dto.params.beginTime},'%y%m%d')`
    if dto.params.endTime != '':
        ` AND date_format(create_time,'%y%m%d') <= date_format(#{dto.params.endTime},'%y%m%d')`
    ` order by create_time desc`"#);


#[html_sql(r#"
        <select id="select_db_table_list_by_names">  select TABLE_NAME table_name, TABLE_COMMENT table_comment, CREATE_TIME create_time, UPDATE_TIME update_time from information_schema.tables`
        ` where table_name NOT LIKE 'qrtz_%' and table_name NOT LIKE 'gen_%' and table_schema = (select database())`
        ` and table_name in`
        <foreach collection="table_names" item="name" open="(" separator="," close=")">
        ` #{name}`
        </foreach></select>"#)]
pub async fn select_db_table_list_by_names(rb: &dyn Executor, table_names: &Vec<&str>) -> Result<Vec<GenTable>, Error> {
    impled!()
}
