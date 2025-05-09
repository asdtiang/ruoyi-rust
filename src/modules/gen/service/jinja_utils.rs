use crate::gen::domain::vo::table::GenTableGenVO;
use crate::gen::domain::vo::table_column::GenTableColumnGenVO;
use crate::utils::string;
use minijinja::context;
use rbatis::rbatis_codegen::ops::AsProxy;
use std::collections::HashSet;
use crate::gen::service::{gen_constants, gen_utils};
//不打算支持子表，只支持连接表！

// package com.ruoyi.generator.util;
//
// import com.alibaba.fastjson2.JSON;
// import com.alibaba.fastjson2.JSONObject;
// import com.ruoyi.common.constant.gen_constants;
// import com.ruoyi.common.utils.DateUtils;
// import com.ruoyi.common.utils.StringUtils;
// import com.ruoyi.generator.domain.GenTable;
// import com.ruoyi.generator.domain.GenTableColumn;
// import org.apache.velocity.VelocityContext;
//
// import java.math.BigDecimal;
// import java.util.ArrayList;
// import java.util.HashSet;
// import java.util.List;
// import java.util.Set;
//
// /**
//  * 模板处理工具类
//  *
//  * @author ruoyi
//  */
// public class VelocityUtils
// {
/** 项目空间路径 */
// private static final String PROJECT_PATH = "main/java";
//
// /** mybatis空间路径 */
// private static final String MYBATIS_PATH = "main/resources/mapper";
//
// /** 默认上级菜单，系统工具 */
// private static final String DEFAULT_PARENT_MENU_ID = "3";

/**
 * 设置模板变量信息
 *
 * @return 模板列表
 */
pub fn prepare_context(
    gen_table: GenTableGenVO,
    columns:  Vec<GenTableColumnGenVO>,
) -> minijinja::value::Value {
    let mut table = gen_table;
    if table.function_name.is_none() {
        table.function_name = Some("【请填写功能名称】".to_string());
    }

    let permission_prefix = get_permission_prefix(
        &table.function_name.clone().unwrap_or_default(),
        &table.business_name.clone().unwrap_or_default(),
    );

    let use_list = get_use_list(&columns);
    let dicts = get_dicts(&columns);
    let mut pk_column = None;
    columns.iter().for_each(|c| {
        if c.is_pk {
            pk_column = Some(c);
        }
    });
    let columns = set_validation_info(&columns);
    //velocityContext.put("datetime", DateUtils.getDate());

    // setMenuVelocityContext(velocityContext, table);
    // if (gen_constants::TPL_TREE.eq(tplCategory))
    // {
    //     setTreeVelocityContext(velocityContext, table);
    // }
    // if (gen_constants::TPL_SUB.eq(tplCategory))
    // {
    //     setSubVelocityContext(velocityContext, table);
    // }
    context! {
        table=>table,
        columns=>columns,
        pkColumn=>pk_column,
        permissionPrefix=>permission_prefix,
        useList=>use_list,
        dicts=>dicts,
    }
}

//     pub fn void setMenuVelocityContext(VelocityContext context, GenTable genTable)
//     {
//         String options = genTable.getOptions();
//         JSONObject paramsObj = JSON.parseObject(options);
//         String parentMenuId = getParentMenuId(paramsObj);
//         context.put("parentMenuId", parentMenuId);
//     }
//
//     pub fn void setTreeVelocityContext(VelocityContext context, GenTable genTable)
//     {
//         String options = genTable.getOptions();
//         JSONObject paramsObj = JSON.parseObject(options);
//         String treeCode = getTreecode(paramsObj);
//         String treeParentCode = getTreeParentCode(paramsObj);
//         String treeName = getTreeName(paramsObj);
//
//         context.put("treeCode", treeCode);
//         context.put("treeParentCode", treeParentCode);
//         context.put("treeName", treeName);
//         context.put("expandColumn", getExpandColumn(genTable));
//         if (paramsObj.containsKey(gen_constants::TREE_PARENT_CODE))
//         {
//             context.put("tree_parent_code", paramsObj.getString(gen_constants::TREE_PARENT_CODE));
//         }
//         if (paramsObj.containsKey(gen_constants::TREE_NAME))
//         {
//             context.put("tree_name", paramsObj.getString(gen_constants::TREE_NAME));
//         }
//     }
//
//     pub fn void setSubVelocityContext(VelocityContext context, GenTable genTable)
//     {
//         GenTable subTable = genTable.getSubTable();
//         String subTableName = genTable.getSubTableName();
//         String subTableFkName = genTable.getSubTableFkName();
//         String subClassName = genTable.getSubTable().getClassName();
//         String subTableFkClassName = StringUtils.convertToCamelCase(subTableFkName);
//
//         context.put("subTable", subTable);
//         context.put("subTableName", subTableName);
//         context.put("subTableFkName", subTableFkName);
//         context.put("subTableFkClassName", subTableFkClassName);
//         context.put("subTableFkclassName", StringUtils.uncapitalize(subTableFkClassName));
//         context.put("subClassName", subClassName);
//         context.put("subclassName", StringUtils.uncapitalize(subClassName));
//         context.put("subuse_list", getuse_list(genTable.getSubTable()));
//     }
//
//     /**
//      * 获取模板信息
//      * @param tplCategory 生成的模板
//      * @param tplWebType 前端类型
//      * @return 模板列表
//      */
//     pub fn List<String> getTemplateList(String tplCategory, String tplWebType)
//     {
//         String useWebType = "vm/vue";
//         if ("element-plus".eq(tplWebType))
//         {
//             useWebType = "vm/vue/v3";
//         }
//         List<String> templates = new ArrayList<String>();
//         templates.add("vm/java/domain.java.vm");
//         templates.add("vm/java/mapper.java.vm");
//         templates.add("vm/java/service.java.vm");
//         templates.add("vm/java/serviceImpl.java.vm");
//         templates.add("vm/java/controller.java.vm");
//         templates.add("vm/xml/mapper.xml.vm");
//         templates.add("vm/sql/sql.vm");
//         templates.add("vm/js/api.js.vm");
//         if (gen_constants::TPL_CRUD.eq(tplCategory))
//         {
//             templates.add(useWebType + "/index.vue.vm");
//         }
//         else if (gen_constants::TPL_TREE.eq(tplCategory))
//         {
//             templates.add(useWebType + "/index-tree.vue.vm");
//         }
//         else if (gen_constants::TPL_SUB.eq(tplCategory))
//         {
//             templates.add(useWebType + "/index.vue.vm");
//             templates.add("vm/java/sub-domain.java.vm");
//         }
//         return templates;
//     }
//
//     /**
//      * 获取文件名
//      */
//     pub fn String getFileName(String template, GenTable genTable)
//     {
//         // 文件名称
//         String fileName = "";
//         // 包路径
//         String packageName = genTable.getPackageName();
//         // 模块名
//         String moduleName = genTable.getModuleName();
//         // 大写类名
//         String className = genTable.getClassName();
//         // 业务名称
//         String businessName = genTable.getBusinessName();
//
//         String javaPath = PROJECT_PATH + "/" + StringUtils.replace(packageName, ".", "/");
//         String mybatisPath = MYBATIS_PATH + "/" + moduleName;
//         String vuePath = "vue";
//
//         if (template.contains("domain.java.vm"))
//         {
//             fileName = StringUtils.format("{}/domain/{}.java", javaPath, className);
//         }
//         if (template.contains("sub-domain.java.vm") && StringUtils.eq(gen_constants::TPL_SUB, genTable.getTplCategory()))
//         {
//             fileName = StringUtils.format("{}/domain/{}.java", javaPath, genTable.getSubTable().getClassName());
//         }
//         else if (template.contains("mapper.java.vm"))
//         {
//             fileName = StringUtils.format("{}/mapper/{}Mapper.java", javaPath, className);
//         }
//         else if (template.contains("service.java.vm"))
//         {
//             fileName = StringUtils.format("{}/service/I{}Service.java", javaPath, className);
//         }
//         else if (template.contains("serviceImpl.java.vm"))
//         {
//             fileName = StringUtils.format("{}/service/impl/{}ServiceImpl.java", javaPath, className);
//         }
//         else if (template.contains("controller.java.vm"))
//         {
//             fileName = StringUtils.format("{}/controller/{}Controller.java", javaPath, className);
//         }
//         else if (template.contains("mapper.xml.vm"))
//         {
//             fileName = StringUtils.format("{}/{}Mapper.xml", mybatisPath, className);
//         }
//         else if (template.contains("sql.vm"))
//         {
//             fileName = businessName + "Menu.sql";
//         }
//         else if (template.contains("api.js.vm"))
//         {
//             fileName = StringUtils.format("{}/api/{}/{}.js", vuePath, moduleName, businessName);
//         }
//         else if (template.contains("index.vue.vm"))
//         {
//             fileName = StringUtils.format("{}/views/{}/{}/index.vue", vuePath, moduleName, businessName);
//         }
//         else if (template.contains("index-tree.vue.vm"))
//         {
//             fileName = StringUtils.format("{}/views/{}/{}/index.vue", vuePath, moduleName, businessName);
//         }
//         return fileName;
//     }
//
//     /**
//      * 获取包前缀
//      *
//      * @param packageName 包名称
//      * @return 包前缀名称
//      */
//     pub fn String getPackagePrefix(String packageName)
//     {
//         int lastIndex = packageName.lastIndexOf(".");
//         return StringUtils.substring(packageName, 0, lastIndex);
//     }
//
/**
 * 根据列类型获取导入包
 *
 * @param genTable 业务表对象
 * @return 返回需要导入的包列表
 */
pub fn get_use_list(columns: &Vec<GenTableColumnGenVO>) -> HashSet<String> {
    // GenTable subGenTable = genTable.getSubTable();
    let mut use_list = HashSet::new();
    // if (StringUtils.isNotNull(subGenTable))
    // {
    //     use_list.insert("java.util.List");
    // }
    for column in columns {
        let binding = column.java_type.clone().unwrap_or_default();
        let java_type = binding.as_str();
        if !column.is_pk {
            if gen_constants::TYPE_DATE.eq(java_type) {
                use_list.insert("java.sql.Date".to_string());
                use_list.insert("com.fasterxml.jackson.annotation.JsonFormat".to_string());
            } else if gen_constants::TYPE_TIMESTAMP.eq(java_type) {
                use_list.insert("java.sql.Timestamp".to_string());
                use_list.insert("com.fasterxml.jackson.annotation.JsonFormat".to_string());
            } else if gen_constants::TYPE_TIME.eq(java_type) {
                use_list.insert("java.sql.Time".to_string());
                use_list.insert("com.fasterxml.jackson.annotation.JsonFormat".to_string());
            } else if gen_constants::TYPE_BIGDECIMAL.eq(java_type) {
                use_list.insert("java.math.BigDecimal".to_string());
            }
        }
        //支持json
        if java_type.eq(gen_constants::TYPE_OBJECT_JSON) {
            use_list.insert("com.fasterxml.jackson.databind.node.ObjectNode".to_string());
        } else if java_type.eq(gen_constants::TYPE_ARRAY_JSON) {
            use_list.insert("com.fasterxml.jackson.databind.node.ArrayNode".to_string());
        }

        if java_type.eq(gen_constants::TYPE_STRING)
            && column.max_length.is_some_and(|i| i > 0)
            && column.is_required
        {
            use_list.insert("javax.validation.constraints.NotBlank".to_string());
            use_list.insert("javax.validation.constraints.Size".to_string());
        } else if column.is_required {
            use_list.insert("javax.validation.constraints.NotNull".to_string());
        } else if java_type.eq(gen_constants::TYPE_BIGDECIMAL) {
            use_list.insert("javax.validation.constraints.Digits".to_string());
        } else if java_type.eq(gen_constants::TYPE_INTEGER) {
            use_list.insert("javax.validation.constraints.Size".to_string());
        } else if java_type.eq(gen_constants::TYPE_LONG) {
            use_list.insert("javax.validation.constraints.Max".to_string());
            use_list.insert("javax.validation.constraints.Min".to_string());
        }
        // if (!StringUtils.isEmpty(column.more("pattern"))) {
        //     use_list.insert("javax.validation.constraints.Pattern");
        // }
    }
    use_list
}

/**
 * 根据列类型获取字典组
 *
 * @param genTable 业务表对象
 * @return 返回字典组
 */
pub fn get_dicts(columns: &Vec<GenTableColumnGenVO>) -> String {
    let mut dicts = HashSet::new();
    for column in columns {
        let dict_type = column.dict_type.clone().unwrap_or_default();
        if !column.is_pk && dict_type.len() > 0 {
            let html_type = column.html_type.clone().unwrap_or_default();
            if html_type.eq(gen_constants::HTML_SELECT)
                || html_type.eq(gen_constants::HTML_RADIO)
                || html_type.eq(gen_constants::HTML_CHECKBOX)
            {
                dicts.insert(format!("'{dict_type}'"));
            }
        }
    }
    let a = dicts.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    a.join(", ")
}
pub fn set_validation_info(columns: &Vec<GenTableColumnGenVO>) -> Vec<GenTableColumnGenVO> {
    let new_column = columns
        .into_iter()
        .map(|column| {
            let mut column = column.clone();
            let java_type = column.java_type.clone().unwrap_or_default();
            let column_type = column.column_type.clone().unwrap_or_default();
            let html_type = column.html_type.clone().unwrap_or_default();
            let more = column.more.clone().unwrap_or_default();
            println!("{more:?}");
            if java_type.eq(gen_constants::TYPE_STRING) {
                let mut column_length = gen_utils::get_column_length(&column_type);
                if more.get("utf").is_some_and(|v|v.as_str().unwrap_or_default().eq("1")){
                    println!("utf");
                    column_length = column_length /2;
                }
                column.max_length = Some(column_length);
            } else if html_type.eq(gen_constants::HTML_NUMBER) {
                match more.get("min") {
                    //默认为0
                    None => {
                        column.min = Some(0);
                    }
                    Some(e) => {
                        column.min = Some(e.as_u64().unwrap().usize());
                    }
                }
                match more.get("max") {
                    Some(e) => {
                        column.max = Some(e.as_u64().unwrap().usize());
                    }
                    _ => {}
                }

                let str = string::substring_between(&column_type, "(", ")");
                let str = str
                    .split(",")
                    .map(|s| s.parse::<usize>())
                    .collect::<Vec<_>>();
                if java_type.eq(gen_constants::TYPE_BIGDECIMAL) {
                    if str.len() >= 2 {
                        column.precision = str[1].clone().ok();
                    }
                    column.max_length = Some(
                        str[1].clone().unwrap_or_default() - str[1].clone().unwrap_or_default(),
                    );
                    //todo
                    // BigDecimal bd = new BigDecimal("1E+" + column.getMaxLength());
                    // BigDecimal pr = new BigDecimal("1E+" + column.getPrecision());
                    // column.setMaxBigDecimal(bd.subtract(new BigDecimal(1).divide(pr)));
                }
                // 如果是整形
                else if java_type.eq(gen_constants::TYPE_INTEGER) {
                    if str.len() >= 1 {
                        column.max_length = str[0].clone().ok();
                    }
                    if column.max.is_none() {
                        //todo 采用更精确的算法
                        column.max = Some(1_000);
                    }
                }
                // 长整形
                else if java_type.eq(gen_constants::TYPE_LONG) {
                    if str.len() >= 1 {
                        column.max_length = str[0].clone().ok();
                    }
                    if column.max.is_none() {
                        //todo 采用更精确的算法
                        column.max = Some(10_000);
                    }
                }
            }
            column
        })
        .collect::<Vec<_>>();
    new_column
}
/**
 * 获取权限前缀
 *
 * @param module_name 模块名称
 * @param business_name 业务名称
 * @return 返回权限前缀
 */
pub fn get_permission_prefix(module_name: &str, business_name: &str) -> String {
    format!("{}:{}", module_name, business_name)
}

//     /**
//      * 获取上级菜单ID字段
//      *
//      * @param paramsObj 生成其他选项
//      * @return 上级菜单ID字段
//      */
//     pub fn String getParentMenuId(JSONObject paramsObj)
//     {
//         if (StringUtils.isNotEmpty(paramsObj) && paramsObj.containsKey(gen_constants::PARENT_MENU_ID)
//                 && StringUtils.isNotEmpty(paramsObj.getString(gen_constants::PARENT_MENU_ID)))
//         {
//             return paramsObj.getString(gen_constants::PARENT_MENU_ID);
//         }
//         return DEFAULT_PARENT_MENU_ID;
//     }
//
//     /**
//      * 获取树编码
//      *
//      * @param paramsObj 生成其他选项
//      * @return 树编码
//      */
//     pub fn String getTreecode(JSONObject paramsObj)
//     {
//         if (paramsObj.containsKey(gen_constants::TREE_CODE))
//         {
//             return StringUtils.toCamelCase(paramsObj.getString(gen_constants::TREE_CODE));
//         }
//         return StringUtils.EMPTY;
//     }
//
//     /**
//      * 获取树父编码
//      *
//      * @param paramsObj 生成其他选项
//      * @return 树父编码
//      */
//     pub fn String getTreeParentCode(JSONObject paramsObj)
//     {
//         if (paramsObj.containsKey(gen_constants::TREE_PARENT_CODE))
//         {
//             return StringUtils.toCamelCase(paramsObj.getString(gen_constants::TREE_PARENT_CODE));
//         }
//         return StringUtils.EMPTY;
//     }
//
//     /**
//      * 获取树名称
//      *
//      * @param paramsObj 生成其他选项
//      * @return 树名称
//      */
//     pub fn String getTreeName(JSONObject paramsObj)
//     {
//         if (paramsObj.containsKey(gen_constants::TREE_NAME))
//         {
//             return StringUtils.toCamelCase(paramsObj.getString(gen_constants::TREE_NAME));
//         }
//         return StringUtils.EMPTY;
//     }
//
//     /**
//      * 获取需要在哪一列上面显示展开按钮
//      *
//      * @param genTable 业务表对象
//      * @return 展开按钮列序号
//      */
//     pub fn int getExpandColumn(GenTable genTable)
//     {
//         String options = genTable.getOptions();
//         JSONObject paramsObj = JSON.parseObject(options);
//         String treeName = paramsObj.getString(gen_constants::TREE_NAME);
//         int num = 0;
//         for (GenTableColumn column : genTable.getColumns())
//         {
//             if (column.isList())
//             {
//                 num++;
//                 String columnName = column.getColumnName();
//                 if (columnName.eq(treeName))
//                 {
//                     break;
//                 }
//             }
//         }
//         return num;
//     }
// }
