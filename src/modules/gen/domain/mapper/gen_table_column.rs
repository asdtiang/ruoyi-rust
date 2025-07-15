use macros::page_request;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql, impl_select, pysql_select_page};
use rbs::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GenTableColumn {
    // 编号
    pub column_id: Option<String>,

    // 归属表编号
    pub table_id: Option<String>,

    // 列名称
    pub column_name: Option<String>,

    // 列描述
    pub column_comment: Option<String>,

    // 列类型
    pub column_type: Option<String>,

    // JAVA类型
    pub java_type: Option<String>,

    // JAVA字段名
    pub java_field: Option<String>,

    // 是否主键（1是）
    pub is_pk: Option<char>,

    // 是否自增（1是）
    pub is_increment: Option<char>,

    // 是否必填（1是）
    pub is_required: Option<char>,

    // 是否为插入字段（1是）
    pub is_insert: Option<char>,

    // 是否编辑字段（1是）
    pub is_edit: Option<char>,

    // 是否列表字段（1是）
    pub is_list: Option<char>,
    //  是否详情字段（1是）
    pub is_detail: Option<char>,

    //是否导出字段（1是）
    pub is_export: Option<char>,

    // 是否可排序字段（1是）
    pub is_sortable: Option<char>,

    //是否查询字段（1是）
    pub is_query: Option<char>,

    // 查询方式（EQ等于、NE不等于、GT大于、LT小于、LIKE模糊、BETWEEN范围）
    pub query_type: Option<String>,

    // 显示类型（input文本框、textarea文本域、select下拉框、checkbox复选框、radio单选框、datetime日期控件、image图片上传控件、upload文件上传控件、editor富文本控件）
    pub html_type: Option<String>,

    // 字典类型
    pub dict_type: Option<String>,

    // 排序
    pub sort: Option<u32>,

    //更多设置
    pub more: Option<serde_json::Value>,

    //默认值
    pub def_val: Option<String>,

    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

crud!(GenTableColumn {});

#[page_request(params)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TablePageDTO {
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
}
impl_select!(GenTableColumn{select_columns_by_table_id(table_id: &str) =>
    "`where table_id= #{table_id}`
     ` order by sort`"});

pysql_select_page!(select_db_table_list(dto:&TablePageDTO) -> GenTableColumn =>
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
    if do_count == false:
        ` order by create_time desc`
        ` limit ${dto.pageNo - 1},${dto.pageSize}`"#);

#[html_sql(r#"
      <select id="select_db_table_columns_by_name">
      select COLUMN_NAME column_name, (case when (is_nullable = 'no' && column_key != 'PRI') then '1' else '0' end) as is_required, (case when column_key = 'PRI' then '1' else '0' end) as is_pk, ordinal_position as sort,COLUMN_COMMENT column_comment, (case when extra = 'auto_increment' then '1' else '0' end) as is_increment,COLUMN_TYPE column_type,column_default as def_val
      from information_schema.columns where table_schema = (select database()) and table_name = (#{table_name})
      order by ordinal_position
      </select>"#)]
pub async fn select_db_table_columns_by_name(
    rb: &dyn Executor,
    table_name: &str,
) -> Result<Vec<GenTableColumn>, Error> {
    impled!()
}
