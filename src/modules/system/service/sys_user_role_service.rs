use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::system::domain::dto::{UserPageDTO, UserRoleDTO, UserRolePageDTO};
use crate::system::domain::mapper::sys_role::SysRole;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::user::SysUserVO;
use rbatis::{field_name, Page};
use rbs::to_value;

///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    pub async fn page(&self, arg: &UserRolePageDTO,login_user_key:&str) -> Result<Page<SysUserVO>> {
        let vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg),login_user_key)
            .await?;

        Ok(vo)
    }

    pub async fn add(&self, arg: UserRoleDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        let user_role = SysUserRole::from(arg);
        self.remove_by_user_id(user_id.as_str()).await?;
        Ok(SysUserRole::insert(pool!(), &user_role).await?.rows_affected)
    }


    pub async fn add_user_roles(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        let rows = role_ids.into_iter().map(|r_id| SysUserRole {
            user_id: user_id.to_string().into(),
            role_id: r_id.to_string().into(),
        }).collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20)
            .await?.rows_affected)
    }

    pub async fn add_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.into_iter().map(|u_id| SysUserRole {
            user_id: u_id.to_string().into(),
            role_id: role_id.to_string().into(),
        }).collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20)
            .await?
            .rows_affected)
    }


    pub async fn remove(&self, user_role: &SysUserRole) -> Result<u64> {
        let res =
            pool!().exec("delete from sys_user_role where user_id=? and role_id=?",
                         vec![to_value!(user_role.user_id.as_ref().unwrap()), to_value!(user_role.role_id.as_ref().unwrap())]).await.unwrap();
        Ok(res.rows_affected)
    }
    pub async fn remove_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.into_iter().map(|u_id| SysUserRole {
            user_id: u_id.to_string().into(),
            role_id: role_id.to_string().into(),
        }).collect::<Vec<_>>();

        let mut cnt = 0;
        for r in rows {
            let res = self.remove(&r).await;
            cnt = cnt + res.unwrap();
        }
        Ok(cnt)
    }
    pub async fn remove_by_role_id(&self, role_id: &String) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.role_id), role_id)
                .await?
                .rows_affected,
        )
    }


    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?
                .rows_affected,
        )
    }


    pub async fn reset_through_user_id(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
            .await?
            .rows_affected;
        self.add_user_roles(user_id, role_ids).await
    }
    pub async fn find_roles_by_user_id(
        &self,
        user_id: &str
    ) -> Result<Option<Vec<SysRole>>> {
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?;

        let role_ids = &rbatis::table_field_vec!(&user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
        Ok(Some(roles))
    }
}
