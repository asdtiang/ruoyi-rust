use crate::config::global_constants::{
    ADMIN_NAME, CHAR_FALSE,
};
use crate::system::domain::dto::MenuPageDTO;
use rbatis::field_name;
use rbs::to_value;
use std::collections::BTreeMap;

use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::modules::system::constants::{INNER_LINK, LAYOUT, PARENT_VIEW, TYPE_DIR, TYPE_MENU};
use crate::pool;
use crate::system::domain::mapper::sys_menu::SysMenu;
use crate::system::domain::vo::{MenuTreeSelectVO, MetaVO, RouterVO, SysMenuVO, UserCache};
use crate::utils::string::capitalize;

const RES_MENU_KEY: &'static str = "sys_menu:all";

/// Menu service
pub struct SysMenuService {}

impl SysMenuService {
    pub async fn query_menu(&self, query: &MenuPageDTO) -> Result<Vec<SysMenuVO>> {
        let res: Vec<SysMenuVO> = SysMenu::query_menu(pool!(), query)
            .await?
            .into_iter()
            .map(|m| SysMenuVO::from(m))
            .collect();
        Ok(res)
    }
    /// Find the menu array
    pub async fn all(&self) -> Result<Vec<SysMenu>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysMenu>>>(RES_MENU_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[ruoyi_rust] get from redis:{}", RES_MENU_KEY);
        }
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        Ok(arr)
    }

    ///menu details
    pub async fn detail(&self, menu_id: &str) -> Result<SysMenu> {
        let menu = SysMenu::select_by_column(pool!(), field_name!(SysMenu.menu_id), menu_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?}！", menu_id)))?;
        Ok(menu)
    }
    pub async fn add(&self, menu: SysMenu) -> Result<u64> {
        let result = Ok(SysMenu::insert(pool!(), &menu).await?.rows_affected);
        self.update_cache().await?;
        result
    }

    pub async fn update(&self, menu: SysMenu) -> Result<u64> {
        let result = SysMenu::update_by_column(pool!(), &menu, "menu_id").await?;
        self.update_cache().await?;
        Ok(result.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysMenu::select_by_column(pool!(), "menu_id", id).await?;

        if trash.len() == 1 {
            let count: u64 = pool!()
                .query_decode(
                    "select count(1) as count from sys_menu where parent_id =?",
                    vec![to_value!(id)],
                )
                .await?;
            if count > 0 {
                return Err(Error::from("存在子菜单,不允许删除！"));
            }
        } else {
            return Err(Error::from(format!("菜单id{}不存在！", id)));
        }
        let num = SysMenu::delete_by_column(pool!(), "menu_id", id)
            .await?
            .rows_affected;
        CONTEXT.sys_trash_service.add("sys_menu", &trash).await?;

        self.update_cache().await?;
        Ok(num)
    }

    pub async fn get_menu_list_by_user_id(&self, user_id: &String) -> Result<Vec<SysMenu>> {
        let res: Option<Vec<SysMenu>> = pool!().query_decode("
      select distinct m.menu_id, m.parent_id, m.menu_name, m.path, m.component, m.`query`, m.visible, m.status, ifnull(m.perms,'') as perms, m.is_frame, m.is_cache, m.menu_type, m.icon, m.order_num, m.create_time
		from sys_menu m
		left join sys_role_menu rm on m.menu_id = rm.menu_id
		left join sys_user_role ur on rm.role_id = ur.role_id
		left join sys_role ro on ur.role_id = ro.role_id
		where ur.user_id = ?
		order by m.parent_id, m.order_num
       ", vec![to_value!(user_id)])
            .await?;
        Ok(res.unwrap_or_default())
    }

    pub async fn update_cache(&self) -> Result<Vec<SysMenu>> {
        let all = SysMenu::select_all_order_num(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_MENU_KEY, &all).await?;
        Ok(all)
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysMenu>> {
        let all = self.all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.menu_id.clone().unwrap_or_default(), x);
        }
        Ok(result)
    }

    pub fn finds_menu(
        &self,
        ids: &Vec<String>,
        all_menus: &BTreeMap<String, SysMenu>,
    ) -> Vec<SysMenu> {
        let mut res = vec![];
        //filter res id
        for (k, v) in all_menus {
            for x in ids {
                if k.eq(x) {
                    res.push(v.clone());
                    break;
                }
            }
        }
        res
    }

    //变成id 和label
    pub async fn tree_select(&self,login_user_key:&str) -> Result<Vec<MenuTreeSelectVO>> {
        let user_cache = CONTEXT
            .sys_user_service
            .get_user_cache_by_token(login_user_key.to_string())
            .await?;
        let menus = if user_cache.user_name == ADMIN_NAME {
            CONTEXT.sys_menu_service.all().await?
        } else {
            CONTEXT
                .sys_menu_service
                .get_menu_list_by_user_id(&user_cache.id)
                .await?
        };

        let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(menus)?;


        Ok(self.build_tree_left_id_label(&menu_tree)?)
    }
    //只剩下id 和label
    pub fn build_tree_left_id_label(&self,menu_tree: &Vec<SysMenuVO>) -> Result<Vec<MenuTreeSelectVO>> {
        let mut d = vec![];
        for SysMenuVO {
            menu_id, menu_name,children, ..
        } in menu_tree
        {
            let mut t = MenuTreeSelectVO {
                id: menu_id.clone(),
                label: menu_name.clone(),
                children: None,
            };
            if children.is_some() {
                t.children = Some(self.build_tree_left_id_label(&children.clone().unwrap())?);
            }
            d.push(t);
        }
        Ok(d)
    }
    ///An menus array with a hierarchy
    pub fn build_menu_tree(&self, all_menus: Vec<SysMenu>) -> Result<Vec<SysMenuVO>> {

        let all_menus=all_menus.into_iter().map(|m|SysMenuVO::from(m)).collect::<Vec<SysMenuVO>>();
        //find tops
        let mut parents = vec![];
        for item in &all_menus {
            let item=item.clone();
            if item.is_parent() {
                parents.push(item);
            }
        }
        //find child
        parents.sort_by(|a, b| a.order_num.cmp(&b.order_num));
        for mut parent in &mut parents {
            self.loop_find_children(&mut parent, &all_menus);
        }
        Ok(parents)
    }

    ///Loop to find the parent-child associative relation array
     fn loop_find_children(&self, parent: &mut SysMenuVO, all_menus: &Vec<SysMenuVO>) {
        let mut children = vec![];
        for item in all_menus {
            if !item.is_parent() && item.parent_id == parent.menu_id {
                let mut parent_ =item.clone();
                self.loop_find_children(&mut parent_, all_menus);
                children.push(parent_);
            }
        }
        if !children.is_empty() {
            children.sort_by(|a, b| a.order_num.cmp(&b.order_num));
            parent.children = Some(children);
        }
    }

    ///生成菜单
    pub async fn get_routers(&self, user_cache: &UserCache) -> Result<Vec<RouterVO>> {
        let all_menus = self.all().await?;
        let filtered_menus = if user_cache.user_name == ADMIN_NAME {
            all_menus
        } else {
            let mut t = vec![];
            for v in all_menus {
                for x in &user_cache.menu_ids {
                    if &v.menu_id.clone().unwrap_or_default() == x {
                        t.push(v.clone());
                    }
                }
            }
            t
        };
        let menu_tree = self.build_menu_tree(filtered_menus)?;
        Ok(self.build_routers(&menu_tree))
    }

    fn build_routers(&self, menus: &Vec<SysMenuVO>) -> Vec<RouterVO> {
        let mut routers = vec![];
        for menu in menus {
            let menu_type = menu.menu_type.clone().unwrap_or_default();
            if menu_type != TYPE_DIR && menu_type != TYPE_MENU {
                continue;
            }
            let mut router = RouterVO {
                name: Some(self.get_route_name(&menu)),
                path: Some(self.get_router_path(&menu)),
                hidden: Some(menu.visible.clone().unwrap() == CHAR_FALSE),
                redirect: None,
                component: Some(self.get_component(&menu)),
                query: menu.query.clone(),
                always_show: None,
                meta: MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(menu.is_cache.unwrap() == CHAR_FALSE),
                    link: None,
                }
                .into(),
                children: vec![],
            };
            let c_menus = menu.children.clone().unwrap();
            if c_menus.len() > 0 && menu.menu_type.unwrap() == TYPE_DIR {
                router.always_show = Some(true);
                router.redirect = Some("noRedirect".to_string());
                router.children = self.build_routers(&c_menus);
            } else if menu.is_menu_frame() {
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(capitalize(&menu.path.clone().unwrap())), //大写
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: menu.component.clone(),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(menu.is_cache.clone().unwrap() == CHAR_FALSE),
                        link: menu.path.clone(),
                    }
                    .into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            } else if menu.is_parent() && menu.is_inner_link() {
                router.meta = MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(false),
                    link: None,
                }
                .into();
                router.path = Some("/".to_string());
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(capitalize(&menu.path.clone().unwrap())), //大写
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: Some(INNER_LINK.to_string()),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(true),
                        link: menu.path.clone(),
                    }
                    .into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            }
            routers.push(router);
        }
        routers
    }
    /**
     * 获取路由名称
     *
     * @param menu 菜单信息
     * @return 路由名称
     */
    fn get_route_name(&self, menu: &SysMenuVO) -> String {
        // 非外链并且是一级目录（类型为目录）
        if menu.is_menu_frame() {
            "".to_string()
        } else {
            capitalize(menu.path.as_deref().unwrap_or_default())
        }
    }
    /**
     * 获取路由地址
     *
     * @param menu 菜单信息
     * @return 路由地址
     */
    fn get_router_path(&self, menu: &SysMenuVO) -> String {
        let mut router_path = menu.path.clone().unwrap();
        // 内链打开外网方式
        if !menu.is_parent() && menu.is_inner_link() {
            // router_path = innerLinkReplaceEach(router_path);
        }
        // 非外链并且是一级目录（类型为目录）

        if menu.is_parent()
            && menu.menu_type.clone().unwrap() == TYPE_DIR
            && menu.is_frame.unwrap() == CHAR_FALSE
        {
            router_path = "/".to_string() + &router_path;
        }
        // 非外链并且是一级目录（类型为菜单）
        else if menu.is_menu_frame() {
            router_path = "/".to_string();
        }
        router_path
    }
    /**
     * 获取组件信息
     *
     * @param menu 菜单信息
     * @return 组件信息
     */
    fn get_component(&self, menu: &SysMenuVO) -> String {
        let old_component = menu.component.as_deref().unwrap_or_default();
        let mut component = LAYOUT;
        if old_component.len() > 0 && !menu.is_menu_frame() {
            component = menu.component.as_ref().unwrap();
        } else if old_component.len() == 0 && !menu.is_parent() && menu.is_inner_link() {
            component = INNER_LINK;
        } else if old_component.len() == 0 && menu.is_parent_view() {
            component = PARENT_VIEW;
        }
        component.to_string()
    }
}
