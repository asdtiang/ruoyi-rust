/**
 * 代码生成通用常量
 *
 * @author ruoyi
 */

/**
 * 单表（增删改查）
 */
pub const  TPL_CRUD :&'static str =  "crud";

/**
 * 树表（增删改查）
 */
pub const  TPL_TREE :&'static str =  "tree";

/**
 * 主子表（增删改查）
 */
pub const  TPL_SUB :&'static str =  "sub";

/**
 * 树编码字段
 */
pub const  TREE_CODE :&'static str =  "treeCode";

/**
 * 树父编码字段
 */
pub const  TREE_PARENT_CODE :&'static str =  "treeParentCode";

/**
 * 树名称字段
 */
pub const  TREE_NAME :&'static str =  "treeName";

/**
 * 上级菜单ID字段
 */
pub const  PARENT_MENU_ID :&'static str =  "parentMenuId";

/**
 * 上级菜单名称字段
 */
pub const  PARENT_MENU_NAME :&'static str =  "parentMenuName";

/**
 * 数据库字符串类型
 */
pub const  COLUMNTYPE_STR : [&str; 4] =  ["char", "varchar", "nvarchar", "varchar2"];

/**
 * 数据库文本类型
 */
pub const COLUMNTYPE_TEXT  : [&str; 4] =  ["tinytext", "text", "mediumtext", "longtext"];

/**
 * 数据库时间类型
 */
pub const COLUMNTYPE_TIME :  [&str; 5] =  ["datetime", "time", "date", "timestamp", "year"];

/**
 * 数据库数字类型
 */
pub const COLUMNTYPE_NUMBER  : [&str; 11] =  ["tinyint", "smallint", "mediumint", "int", "number", "integer",
"bit", "bigint", "float", "double", "decimal"];

/**
 * 页面不需要编辑字段
 */
pub const COLUMNNAME_NOT_EDIT : [&str; 4] =  ["id", "create_by", "create_time", "del_flag"];

/**
 * 页面不需要显示的列表字段
 */
pub const COLUMNNAME_NOT_LIST : [&str; 6] =  ["id", "create_by", "create_time", "del_flag", "update_by",
"update_time"];

/**
 * 页面不需要查询字段
 */
pub const COLUMNNAME_NOT_QUERY  : [&str; 7] =  ["id", "create_by", "create_time", "del_flag", "update_by",
"update_time", "remark"];

/**
 * Entity基类字段
 */
pub const BASE_ENTITY  : [&str; 5] =  ["createBy", "createTime", "updateBy", "updateTime", "remark"];

/**
 * Tree基类字段
 */
pub const TREE_ENTITY  : [&str; 5] =  ["parentName", "parentId", "orderNum", "ancestors", "children"];

/**
 * 文本框
 */
pub const  HTML_INPUT :&'static str =  "input";

/**
 * 文本域
 */
pub const  HTML_TEXTAREA :&'static str =  "textarea";

/**
 * 下拉框
 */
pub const  HTML_SELECT :&'static str =  "select";

/**
 * 单选框
 */
pub const  HTML_RADIO :&'static str =  "radio";

/**
 * 数字框
 */
pub const  HTML_NUMBER :&'static str =  "number";

/**
 * 复选框
 */
pub const  HTML_CHECKBOX :&'static str =  "checkbox";

/**
 * 日期控件
 */
pub const  HTML_DATE :&'static str =  "date";
/**
 * 时间控件
 */
pub const  HTML_TIME :&'static str =  "time";
/**
 * 日期控件
 */
pub const  HTML_DATETIME :&'static str =  "datetime";

/**
 * 图片上传控件
 */
pub const  HTML_IMAGE_UPLOAD :&'static str =  "imageUpload";

/**
 * 文件上传控件
 */
pub const  HTML_FILE_UPLOAD :&'static str =  "fileUpload";

/**
 * 富文本控件
 */
pub const  HTML_EDITOR :&'static str =  "editor";

/**
 * 字符串类型
 */
pub const  TYPE_STRING :&'static str =  "String";

/**
 * 布尔型
 */
pub const  TYPE_BOOLEAN :&'static str =  "bool";
/**
 * 整型
 */
pub const  TYPE_INTEGER :&'static str =  "i32";
/**
 * json类型 ObjectNode
 */
pub const  TYPE_OBJECT_JSON :&'static str =  "serde_json::Value";
/**
 * json 类型 ArrayNode
 */
pub const  TYPE_ARRAY_JSON :&'static str =  "serde_json::Value";
/**
 * char
 */
pub const  TYPE_CHAR :&'static str =  "char";
/**
 * 长整型
 */
pub const  TYPE_LONG :&'static str =  "u64";

/**
 * 浮点型
 */
pub const  TYPE_DOUBLE :&'static str =  "f64";

/**
 * 高精度计算类型，fixme 引用bigdecimal crate
 */
pub const  TYPE_BIGDECIMAL :&'static str =  "f64";

/**
 * 日期类型
 */
pub const  TYPE_DATE :&'static str =  "Date";
/**
 * 日期时间类型
 */
pub const  TYPE_TIMESTAMP :&'static str =  "DateTime";
/**
 * 时间类型
 */
pub const  TYPE_TIME :&'static str =  "Time";

/**
 * 模糊查询
 */
pub const  QUERY_LIKE :&'static str =  "LIKE";

/** 相等查询 */
pub const  QUERY_EQ :&'static str =  "EQ";

/** 需要 */
pub const  REQUIRE  : char =  '1';

/** 不需要 */
pub const  NOT_REQUIRE  : char =  '0';

/**
 * 需要在mod.rs加入pub use xx_xx::*
 */
pub const PUB_USE_NAME  : [&str; 4] =  ["dto", "mapper", "vo", "service"];
