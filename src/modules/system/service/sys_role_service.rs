use crate::context::CONTEXT;
use crate::error::{Error, Result};
use crate::system::domain::dto::{RoleAuthUserPageDTO, RolePageDTO};
use crate::system::domain::mapper::sys_role::SysRole;
use crate::system::domain::mapper::sys_role_menu::SysRoleMenu;
use crate::system::domain::mapper::sys_user;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::{SysRoleVO, SysUserVO};
use crate::{export_excel_service, pool, remove_batch};
use macros::data_scope;
use rbatis::{field_name, Page, PageRequest};
use rbs::to_value;

const RES_KEY: &'static str = "sys_role:all";

///Role of service
pub struct SysRoleService {}

impl SysRoleService {
    #[data_scope(deptAlias = "d", userAlias = "u")]
    pub async fn page(&self, dto: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let data = SysRole::select_page(pool!(), &PageRequest::from(&dto), &dto).await?;
        let page = Page::<SysRoleVO>::from(data);
        Ok(page)
    }
    ///role details
    pub async fn detail(&self, role_id: &str) -> Result<SysRole> {
        let role = SysRole::select_by_column(pool!(), field_name!(SysRole.role_id), role_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} ", role_id)))?;
        Ok(role)
    }

    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRole::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        Ok(all)
    }

    pub async fn add(&self, role: SysRole, menu_ids: Vec<String>) -> Result<u64> {
        let result = SysRole::insert(pool!(), &role).await?.rows_affected;

        if result > 0 && !menu_ids.is_empty() {
            CONTEXT
                .sys_role_menu_service
                .add_role_menus(role.role_id.unwrap(), menu_ids)
                .await?;
        }
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn update(&self, role: SysRole, menu_ids: Vec<String>) -> Result<u64> {
        self.check_role_allowed(&role).await?;

        let result = SysRole::update_by_column(pool!(), &role, field_name!(SysRole.role_id))
            .await?
            .rows_affected;
        if result > 0 {
            let role_id = role.role_id.clone().unwrap();
            CONTEXT.sys_role_menu_service.remove_by_role_id(&role_id).await?;
            if !menu_ids.is_empty() {
                CONTEXT.sys_role_menu_service.add_role_menus(role_id, menu_ids).await?;
            }
        }
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn update_status(&self, role: SysRole,user: &crate::UserCache) -> Result<u64> {
        let role_id = role.role_id.clone().unwrap_or_default();
        let status = role.status.unwrap_or_default();
        self.check_role_allowed(&role).await?;
        self.check_role_data_scope(&role_id,user).await?;
        let res = pool!()
            .exec(
                "update sys_role set status = ? where role_id = ?",
                vec![to_value!(status), to_value!(role_id)],
            )
            .await?;
        self.update_cache().await?;
        Ok(res.rows_affected)
    }
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysRole::select_by_column(pool!(), field_name!(SysRole.role_id), id).await?;
        let result = SysRole::delete_by_column(pool!(), field_name!(SysRole.role_id), id)
            .await?
            .rows_affected;
        if result > 0 {
            CONTEXT.sys_role_menu_service.remove_by_role_id(id).await?;
        }
        CONTEXT.sys_trash_service.add("sys_role", &trash).await?;
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn finds(&self, ids: &Vec<&String>) -> Result<Vec<SysRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRole::select_in_column(pool!(), "role_id", ids).await?)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysRoleVO>> {
        let data = SysRole::select_all(pool!()).await?;
        let mut role_vos = vec![];
        for s in data {
            role_vos.push(SysRoleVO::from(s));
        }
        Ok(role_vos)
    }

    /**
     * 根据用户ID查询角色
     *
     * @param userId 用户ID
     * @return 角色列表
     */

    //查找所有roles，如果用户包含此权限，则flag=true
    pub async fn finds_roles_by_user_id(&self, user_id: &str) -> Result<Vec<SysRoleVO>> {
        let all = SysRole::select_all(pool!()).await?;
        let mut res = vec![];
        let user_roles = SysUserRole::select_by_column(pool!(), "user_id", user_id).await?;
        for r in all {
            let mut r_vo = SysRoleVO::from(r);

            for ur in &user_roles {
                if r_vo.role_id.eq(&ur.role_id) {
                    r_vo.flag = true;
                    break;
                }
            }
            res.push(r_vo);
        }

        res.sort_by(|a, b| a.role_sort.cmp(&b.role_sort));

        Ok(res)
    }
    pub async fn finds_role_ids_by_user_id(&self, user_id: &str) -> Result<Vec<String>> {
        let user_roles = SysUserRole::select_by_column(pool!(), "user_id", user_id).await?;
        let res = user_roles
            .into_iter()
            .map(|ur| ur.role_id.unwrap_or_default())
            .collect::<Vec<_>>();
        Ok(res)
    }

    pub async fn find_role_menu(&self, role_ids: &Vec<&String>) -> Result<Vec<SysRoleMenu>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleMenu::select_in_column(pool!(), "role_id", role_ids).await?)
    }

    #[data_scope(deptAlias = "d", userAlias = "u")]
    pub async fn allocated_user_list_page(&self, dto: &RoleAuthUserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> =
            sys_user::allocated_user_list(pool!(), &PageRequest::from(&dto), &dto).await?;
        let page = Page::<SysUserVO>::from(sys_user_page);
        Ok(page)
    }
    #[data_scope(deptAlias = "d", userAlias = "u")]
    pub async fn unallocated_user_list_page(&self, dto: &RoleAuthUserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> =
            sys_user::unallocated_user_list(pool!(), &PageRequest::from(&dto), &dto).await?;
        let page = Page::<SysUserVO>::from(sys_user_page);
        Ok(page)
    }

    pub async fn check_role_allowed(&self, role: &SysRole) -> Result<bool> {
        if role.role_id.is_some() && role.is_admin() {
            Err(Error::from("不允许操作超级管理员角色"))
        } else {
            Ok(true)
        }
    }
    pub async fn check_role_data_scope(&self, role_id: &str, user_cache:&crate::UserCache) -> Result<bool> {
        if !user_cache.is_admin() {
            let dto = RolePageDTO {
                page_no: None,
                page_size: None,
                role_id: Some(role_id.to_string()),
                role_name: None,
                role_key: None,
                status: None,
                params: None,
            };
            let roles = self.page(&dto, user_cache).await?;
            if roles.records.is_empty() {
                return Err(Error::from("没有权限访问角色数据！"));
            }
        }
        Ok(true)
    }
    pub async fn auth_data_scope(&self, role: &SysRole, dept_ids: &Vec<String>,user: &crate::UserCache) -> Result<bool> {
        self.check_role_allowed(role).await?;
        let role_id = role.role_id.clone().unwrap_or_default();
        self.check_role_data_scope(&role_id, user).await?;
        SysRole::update_by_column(pool!(), &role, field_name!(SysRole.role_id))
            .await?;
        CONTEXT.sys_role_dept_service.remove_by_role_id(&role_id).await?;
        if !dept_ids.is_empty() {
            CONTEXT.sys_role_dept_service.add_role_depts(&role_id, dept_ids).await?;
        }

        Ok(true)
    }
    remove_batch!(role_ids);
    export_excel_service!(RolePageDTO, SysRoleVO, SysRole::select_page);
}
