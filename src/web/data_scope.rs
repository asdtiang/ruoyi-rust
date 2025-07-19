use crate::context::CONTEXT;
use crate::error::Result;
use crate::DataScopeTrait;

/**
 * 全部数据权限
 */
pub const DATA_SCOPE_ALL: char = '1';

/**
 * 自定数据权限
 */
pub const DATA_SCOPE_CUSTOM: char = '2';

/**
 * 部门数据权限
 */
pub const DATA_SCOPE_DEPT: char = '3';

/**
 * 部门及以下数据权限
 */
pub const DATA_SCOPE_DEPT_AND_CHILD: char = '4';

/**
 * 仅本人数据权限
 */
pub const DATA_SCOPE_SELF: char = '5';

/**
 * 数据权限过滤关键字
 */
pub const DATA_SCOPE: &'static str = "dataScope";

pub async fn build_data_scope<T>(dto: &mut T, dept_alias: &str, user_alias: &str,login_user_key:&str) -> Result<bool>
where
    T: DataScopeTrait,
{
    //拼接权限sql前先清空params.dataScope参数防止注入
    dto.clear_data_scope_params();
    let user_cache = CONTEXT.sys_user_service.get_user_cache_by_token(login_user_key.to_string()).await?;
    let user = user_cache.user.unwrap();
    if user.admin {
        dto.set_data_scope_params("");
        return Ok(true);
    }
    let mut sql_string = String::new();
    let mut conditions = vec![];
    let dept_id = user.dept_id.unwrap_or_default();

    //未用到  /**
    //      * 权限字符（用于多个角色匹配符合要求的权限）默认根据权限注解@ss获取，多个权限用逗号分隔开来
    //      */
    //     public String permission() default ""; if (StringUtils.isNotNull(currentUser) && !currentUser.isAdmin())
    // {
    //     String permission = StringUtils.defaultIfEmpty(controllerDataScope.permission(), PermissionContextHolder.getContext());
    //     dataScopeFilter(joinPoint, currentUser, controllerDataScope.deptAlias(),
    //                     controllerDataScope.userAlias(), permission);
    // }
    for role in user_cache.roles {
        let data_scope = role.data_scope.unwrap_or_default();
        if DATA_SCOPE_CUSTOM != data_scope && conditions.contains(&data_scope) {
            continue;
        }
        //未用到 if (StringUtils.isNotEmpty(permission) && StringUtils.isNotEmpty(role.getPermissions())
        //     && !StringUtils.containsAny(role.getPermissions(), Convert.toStrArray(permission)))
        // {
        //     continue;
        // }
        if DATA_SCOPE_ALL == data_scope {
            sql_string = String::new();
            conditions.push(data_scope);
            break;
        } else if DATA_SCOPE_CUSTOM == data_scope {
            sql_string.push_str(&format!(
                " OR {}.dept_id IN ( SELECT dept_id FROM sys_role_dept WHERE role_id = '{}' ) ",
                dept_alias,
                role.role_id.unwrap_or_default()
            ));
        } else if DATA_SCOPE_DEPT == data_scope {
            sql_string.push_str(&format!(" OR {}.dept_id = '{}' ", dept_alias, dept_id));
        } else if DATA_SCOPE_DEPT_AND_CHILD == data_scope {
            sql_string.push_str(&format!(
                " OR {}.dept_id IN ( SELECT dept_id FROM sys_dept WHERE dept_id = '{}' or find_in_set( '{}' , ancestors ) )",
                dept_alias, dept_id, dept_id));
        } else if DATA_SCOPE_SELF == data_scope {
            if user_alias.len() > 0 {
                sql_string.push_str(&format!(
                    " OR {}.user_id = '{}' ",
                    user_alias,
                    user.user_id.clone().unwrap_or_default()
                ));
            } else {
                // 数据权限为仅本人且没有userAlias别名不查询任何数据
                sql_string.push_str(&format!(" OR {}.dept_id = 0 ", dept_alias));
            }
        }
        conditions.push(data_scope);
    }

    // 多角色情况下，所有角色都不包含传递过来的权限字符，这个时候sqlString也会为空，所以要限制一下,不查询任何数据
    if conditions.len() == 0 {
        sql_string.push_str(&format!(" OR {}.dept_id = 0 ", dept_alias));
    }

    if sql_string.len() > 0 {
        dto.set_data_scope_params(&format!(" AND ({})", sql_string[4..].to_string()));
    }
    println!("sql_string:{}", sql_string);
    Ok(true)
}
