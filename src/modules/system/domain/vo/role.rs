use macros::Export;
use  crate::system::domain::mapper::sys_role::SysRole;
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Export)]
pub struct SysRoleVO {
    pub admin: bool,
    /** 角色ID */
    #[excel( "角色序号")]
    pub role_id: Option<String>,
    /** 角色名称 */
    #[excel( "角色名称")]
    pub role_name: Option<String>,
    /** 角色权限 */
    #[excel( "角色权限")]
    pub role_key: Option<String>,
    /** 角色排序 */
    #[excel( "角色排序")]
    pub role_sort: Option<u32>,
    /** 数据范围（1：所有数据权限；2：自定义数据权限；3：本部门数据权限；4：本部门及以下数据权限；5：仅本人数据权限） */
    #[excel( "数据范围", readConverterExp = "1=所有数据权限,2=自定义数据权限,3=本部门数据权限,4=本部门及以下数据权限,5=仅本人数据权限")]
    pub data_scope: Option<char>,
    /** 菜单树选择项是否关联显示（ 0：父子不互相关联显示 1：父子互相关联显示） */
    pub menu_check_strictly: Option<bool>,
    /** 部门树选择项是否关联显示（0：父子不互相关联显示 1：父子互相关联显示 ） */
    pub dept_check_strictly: Option<bool>,
    /** 角色状态（0正常 1停用） */
    #[excel( "角色状态", readConverterExp = "0=正常,1=停用")]
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
    /** 用户是否存在此角色标识 默认不存在 */
    pub flag: bool,
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            admin: arg.is_admin(),
            role_id: arg.role_id,
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort,
            data_scope: arg.data_scope,
            menu_check_strictly: arg.menu_check_strictly.eq(&Some('1')).into(),
            dept_check_strictly: arg.dept_check_strictly.eq(&Some('1')).into(),
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
            flag: false,
        }
    }
}
