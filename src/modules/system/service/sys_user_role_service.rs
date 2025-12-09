use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::system::domain::dto::{UserPageDTO, UserRoleDTO, UserRolePageDTO};
use crate::system::domain::mapper::sys_role::SysRole;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::user::SysUserVO;
use macros::{replace_pool, transactional};
use rbatis::{field_name, Page};
use rbs::to_value;

///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    pub async fn page(&self, arg: &UserRolePageDTO, user_cache: &crate::UserCache) -> Result<Page<SysUserVO>> {
        let vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg), user_cache)
            .await?;

        Ok(vo)
    }
    #[replace_pool]
    pub async fn add(&self, arg: UserRoleDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.clone().unwrap_or_default();
        let user_role = SysUserRole::from(arg);
        self.remove_by_user_id_tx(&user_id, tx).await?;
        Ok(SysUserRole::insert(pool!(), &user_role).await?.rows_affected)
    }

    #[replace_pool]
    pub async fn add_user_roles(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        let rows = role_ids
            .into_iter()
            .map(|r_id| SysUserRole {
                user_id: user_id.to_string().into(),
                role_id: r_id.to_string().into(),
            })
            .collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20).await?.rows_affected)
    }
    #[replace_pool(true)]
    pub async fn add_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids
            .into_iter()
            .map(|u_id| SysUserRole {
                user_id: u_id.to_string().into(),
                role_id: role_id.to_string().into(),
            })
            .collect::<Vec<_>>();

        Ok(SysUserRole::insert_batch(pool!(), &rows, 20).await?.rows_affected)
    }

    #[replace_pool(true)]
    pub async fn remove(&self, user_role: &SysUserRole) -> Result<u64> {
        let res = pool!()
            .exec(
                "delete from sys_user_role where user_id=? and role_id=?",
                vec![
                    to_value!(user_role.user_id.clone()),
                    to_value!(user_role.role_id.clone()),
                ],
            )
            .await?;
        Ok(res.rows_affected)
    }
    #[transactional(tx)]
    pub async fn remove_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let mut cnt = 0;
        for user_id in user_ids {
            let r = tx
                .exec(
                    "delete from sys_user_role where user_id=? and role_id=?",
                    vec![to_value!(user_id), to_value!(role_id)],
                )
                .await?
                .rows_affected;
            cnt = cnt + r;
        }
        Ok(1)
    }
    #[replace_pool]
    pub async fn remove_by_role_id(&self, role_id: &String) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.role_id), role_id)
                .await?
                .rows_affected,
        )
    }

    #[replace_pool]
    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserRole::delete_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
                .await?
                .rows_affected,
        )
    }

    #[transactional(tx)]
    pub async fn reset_through_user_id(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        self.reset_through_user_id_tx(user_id, role_ids, &tx).await
    }

    pub async fn reset_through_user_id_tx(
        &self,
        user_id: &str,
        role_ids: &Vec<String>,
        tx: &rbatis::executor::RBatisTxExecutor,
    ) -> Result<u64> {
        SysUserRole::delete_by_column(tx, field_name!(SysUserRole.user_id), user_id)
            .await?
            .rows_affected;
        self.add_user_roles_tx(user_id, role_ids, tx).await
    }
    pub async fn find_roles_by_user_id(&self, user_id: &str) -> Result<Vec<SysRole>> {
        if user_id.is_empty() {
            return Ok(vec![]);
        }

        //todo 要不要变成关联查询
        let user_roles = SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id).await?;

        let role_ids = &rbatis::table_field_vec!(&user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
        Ok(roles)
    }
}
