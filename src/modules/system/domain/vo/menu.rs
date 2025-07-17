use crate::config::global_constants::CHAR_FALSE;
use crate::modules::system::constants::{TYPE_DIR, TYPE_MENU};
use crate::system::domain::mapper::sys_menu::SysMenu;
use rbatis::rbdc::types::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysMenuVO {
    /** 菜单ID */
    pub menu_id: Option<String>,
    /** 菜单名称 */
    pub menu_name: Option<String>,
    /** 父菜单ID */
    pub parent_id: Option<String>,
    /** 显示顺序 */
    pub order_num: Option<u32>,
    /** 路由地址 */
    pub path: Option<String>,
    /** 组件路径 */
    pub component: Option<String>,
    /** 路由参数 */
    pub query: Option<String>,
    /** 是否为外链（0是 1否） */
    pub is_frame: Option<char>,
    /** 是否缓存（0缓存 1不缓存） */
    pub is_cache: Option<char>,
    /** 类型（M目录 C菜单 F按钮） */
    pub menu_type: Option<char>,
    /** 显示状态（0显示 1隐藏） */
    pub visible: Option<char>,
    /** 菜单状态（0正常 1停用） */
    pub status: Option<char>,
    /** 权限字符串 */
    pub perms: Option<String>,
    /** 菜单图标 */
    pub icon: Option<String>,
    pub create_time: Option<DateTime>,
    /** 子菜单 */
    pub children: Option<Vec<SysMenuVO>>,
}

impl From<SysMenu> for SysMenuVO {
    fn from(arg: SysMenu) -> Self {
        Self {
            menu_id: arg.menu_id,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            order_num: arg.order_num,
            path: arg.path,
            component: arg.component,
            query: arg.query,
            is_frame: arg.is_frame,
            is_cache: arg.is_cache,
            menu_type: arg.menu_type,
            visible: arg.visible,
            status: arg.status,
            perms: arg.perms,
            icon: arg.icon,
            create_time: arg.create_time,
            children: vec![].into(),
        }
    }
}

impl SysMenuVO {
    pub fn is_parent(&self) -> bool {
        self.parent_id.eq(&Some("0".to_string()))
    }
    pub fn is_menu_frame(&self) -> bool {
        self.is_parent()
            && self.menu_type.clone().unwrap_or_default() == TYPE_MENU
            && self.is_frame.clone().unwrap_or_default() == CHAR_FALSE
    }
    pub fn is_inner_link(&self) -> bool {
        let path = self.path.clone().unwrap_or_default();
        self.is_frame.clone().unwrap_or_default() == CHAR_FALSE
            && (path.starts_with("http://") || path.starts_with("https://"))
    }
    pub fn is_parent_view(&self) -> bool {
        !self.is_parent() && self.menu_type.clone().unwrap_or_default() == TYPE_DIR
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct MenuTreeSelectVO {
    pub id: Option<String>,
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /** 子菜单 */
    pub children: Option<Vec<MenuTreeSelectVO>>,
}
