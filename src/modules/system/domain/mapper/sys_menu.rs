use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select};

//SysMenu
crud!(SysMenu {});//如何去掉第一个AND
impl_select!(SysMenu{query_menu(dto: &crate::system::domain::dto::MenuPageDTO) =>
"`where 1=1`
    if dto.menuName != '':
      ` and menu_name like #{'%'+dto.menuName+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    ` order by order_num`"});
impl_select!(SysMenu{select_all_order_num() =>
    "` order by order_num`"});


///Permission Menu Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysMenu {
    /** 菜单ID */
    pub menu_id: Option<String>,
    /** 菜单名称 */
    pub menu_name: Option<String>,
    //父id(可空)
    /** 父菜单ID */
    pub parent_id: Option<String>,
    //顺序
    /** 显示顺序 */
    pub order_num: Option<u32>,
    //前端-菜单路径
    /** 路由地址 */
    pub path: Option<String>,
    //组件路径
    /** 组件路径 */
    pub component: Option<String>,
    //组件路径
    /** 路由参数 */
    pub query: Option<String>,
    //是否为外链
    /** 是否为外链（0是 1否） */
    pub is_frame: Option<char>,
    //是否缓存
    /** 是否缓存（0缓存 1不缓存） */
    pub is_cache: Option<char>,
    //菜单类型
    /** 类型（M目录 C菜单 F按钮） */
    pub menu_type: Option<char>,
    //菜单可见
    /** 显示状态（0显示 1隐藏） */
    pub visible: Option<char>,
    //菜单状态
    /** 菜单状态（0正常 1停用） */
    pub status: Option<char>,
    //权限标识
    /** 权限字符串 */
    pub perms: Option<String>,
    //图标
    /** 菜单图标 */
    pub icon: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
