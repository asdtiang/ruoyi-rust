use crate::error::Result;
use crate::pool;
use crate::system::domain::mapper::sys_role_dept::SysRoleDept;
use macros::replace_pool;

/// Role Dept Service
pub struct SysRoleDeptService {}

impl SysRoleDeptService {
    #[replace_pool]
    pub async fn add_role_depts(&self, role_id: &str, dept_ids: &Vec<String>) -> Result<u64> {
        let mut sys_role_dept = vec![];
        for dept_id in dept_ids {
            sys_role_dept.push(SysRoleDept {
                role_id: Some(role_id.to_string()),
                dept_id: Some(dept_id.to_string()),
            });
        }
        Ok(SysRoleDept::insert_batch(pool!(), &sys_role_dept, 20)
            .await?
            .rows_affected)
    }
    #[replace_pool]
    pub async fn add_roles_dept(&self, dept_id: &str, role_ids: Vec<String>) -> Result<u64> {
        let mut sys_role_depts = vec![];
        for role_id in role_ids {
            sys_role_depts.push(SysRoleDept {
                role_id: Some(role_id.clone()),
                dept_id: Some(dept_id.to_string()),
            });
        }
        Ok(SysRoleDept::insert_batch(pool!(), &sys_role_depts, 20)
            .await?
            .rows_affected)
    }

    #[replace_pool]
    pub async fn remove_by_dept_id(&self, dept_id: &str) -> Result<u64> {
        Ok(SysRoleDept::delete_by_column(pool!(), "dept_id", dept_id)
            .await?
            .rows_affected)
    }
    #[replace_pool]
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(SysRoleDept::delete_by_column(pool!(), "role_id", role_id)
            .await?
            .rows_affected)
    }

    pub async fn select_by_role_id(&self, role_id: &str) -> Result<Vec<SysRoleDept>> {
        Ok(SysRoleDept::select_by_column(pool!(), "role_id", role_id)
            .await?)
    }
}
