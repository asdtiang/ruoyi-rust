use crate::system::domain::mapper::sys_menu::SysMenu;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuPageDTO {
    pub menu_name: Option<String>,
    pub status: Option<char>
}

impl Default for MenuPageDTO {
    fn default() -> Self {
        Self {
            menu_name: None,
            status: None,
        }
    }
}
#[derive(Serialize, Deserialize,validator::Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuAddDTO {
    /** 菜单名称 */
    #[validate(custom(function = "crate::string_required", message = "菜单名称不能为空"))]
    #[validate(length(max = 50, message = "菜单名称长度不能超过50个字符"))]
    pub menu_name: Option<String>,
    //父id(可空)
    /** 父菜单ID */
    pub parent_id: Option<String>,
    /** 显示顺序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub order_num: Option<u32>,
    /** 路由地址 */
    #[validate(length(max = 200, message = "路由地址不能超过200个字符"))]
    pub path: Option<String>,
    /** 组件路径 */
    #[validate(length(max = 200, message = "组件路径不能超过255个字符"))]
    pub component: Option<String>,
    /** 路由参数 */
    pub query: Option<String>,
    /** 路由名称 */
    #[validate(length(max = 50, message = "路由名称不能超过50个字符"))]
    pub route_name: Option<String>,
    /** 是否为外链（0是 1否） */
    pub is_frame: Option<char>,
    /** 是否缓存（0缓存 1不缓存） */
    pub is_cache: Option<char>,
    /** 类型（M目录 C菜单 F按钮） */
    #[validate(required(message = "菜单类型不能为空"))]
    pub menu_type: Option<char>,
    /** 显示状态（0显示 1隐藏） */
    pub visible: Option<char>,
    /** 菜单状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    /** 权限字符串 */
    #[validate(length(max = 100, message = "权限标识长度不能超过100个字符"))]
    pub perms: Option<String>,
    /** 菜单图标 */
    pub icon: Option<String>,
    pub remark: Option<String>,
}

impl From<MenuAddDTO> for SysMenu {
    fn from(arg: MenuAddDTO) -> Self {
        SysMenu {
            menu_id:  ObjectId::new().to_string().into(),
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            order_num: arg.order_num,
            path: arg.path,
            component: arg.component,
            query: arg.query,
            route_name: arg.route_name,
            is_frame: arg.is_frame,
            is_cache: arg.is_cache,
            menu_type: arg.menu_type,
            visible: arg.visible,
            status: arg.status,
            perms: arg.perms,
            icon: arg.icon,
            create_by: None,
            create_time: None,

            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, validator::Validate,Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuUpdateDTO {
    /** 菜单ID */
    pub menu_id: Option<String>,
    /** 菜单名称 */
    #[validate(custom(function = "crate::string_required", message = "菜单名称不能为空"))]
    #[validate(length(max = 50, message = "菜单名称长度不能超过50个字符"))]
    pub menu_name: Option<String>,
    /** 父菜单ID */
    pub parent_id: Option<String>,
    /** 显示顺序 */
    #[validate(required(message = "显示顺序不能为空"))]
    pub order_num: Option<u32>,
    /** 路由地址 */
    #[validate(length(max = 200, message = "路由地址不能超过200个字符"))]
    pub path: Option<String>,
    /** 组件路径 */
    #[validate(length(max = 200, message = "组件路径不能超过255个字符"))]
    pub component: Option<String>,
    /** 路由参数 */
    pub query: Option<String>,
    /** 路由名称 */
    #[validate(length(max = 50, message = "路由名称不能超过50个字符"))]
    pub route_name: Option<String>,
    /** 是否为外链（0是 1否） */
    pub is_frame: Option<char>,
    /** 是否缓存（0缓存 1不缓存） */
    pub is_cache: Option<char>,
    /** 类型（M目录 C菜单 F按钮） */
    #[validate(required(message = "菜单类型不能为空"))]
    pub menu_type: Option<char>,
    /** 显示状态（0显示 1隐藏） */
    pub visible: Option<char>,
    /** 菜单状态（0正常 1停用） */
    #[validate(custom(function = "crate::status_char", message = "状态错误"))]
    pub status: Option<char>,
    /** 权限字符串 */
    #[validate(length(max = 100, message = "权限标识长度不能超过100个字符"))]
    pub perms: Option<String>,
    /** 菜单图标 */
    pub icon: Option<String>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

impl From<MenuUpdateDTO> for SysMenu {
    fn from(arg: MenuUpdateDTO) -> Self {
        SysMenu {
            menu_id: arg.menu_id,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            order_num: arg.order_num,
            path: arg.path,
            component: arg.component,
            query: arg.query,
            route_name: arg.route_name,
            is_frame: arg.is_frame,
            is_cache: arg.is_cache,
            menu_type: arg.menu_type,
            visible: arg.visible,
            status: arg.status,
            perms: arg.perms,
            icon: arg.icon,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: None,
        }
    }
}
