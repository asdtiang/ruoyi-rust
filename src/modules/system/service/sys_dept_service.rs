use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::modules::system::constants::{DEPT_DISABLE, DEPT_NORMAL};
use crate::system::domain::dto::DeptQueryDTO;
use crate::system::domain::mapper;
use crate::system::domain::mapper::sys_dept;
use crate::system::domain::mapper::sys_dept::SysDept;
use crate::system::domain::vo::{DeptTreeVO, SysDeptVO};
use crate::{pool, UserCache};
use macros::{data_scope, replace_pool, transactional};
use rbatis::field_name;
use rbs::to_value;
pub struct SysDeptService {}

impl SysDeptService {
    #[data_scope(deptAlias = "d")]
    pub async fn list(&self, arg: &DeptQueryDTO) -> Result<Vec<SysDept>> {
        let data = mapper::sys_dept::select_all_(pool!(), &arg).await?;
        Ok(data)
    }

    pub async fn detail(&self, dept_id: &str, user: &crate::UserCache) -> Result<SysDeptVO> {
        self.check_dept_data_scope(dept_id, user).await?;
        let dept = self.get_dept_by_id(dept_id).await?;
        Ok(SysDeptVO::from(dept))
    }

    pub async fn add(&self, dept: SysDept) -> Result<u64> {
        self.check_dept_name_unique(&&None, &dept.dept_name, &dept.parent_id)
            .await?;

        let parent = self.get_dept_by_id(&dept.parent_id.clone().unwrap()).await?;
        if parent.status.unwrap_or_default() != DEPT_NORMAL {
            return Err(Error::from("部门停用，不允许新增"));
        }
        Ok(SysDept::insert(pool!(), &dept).await?.rows_affected)
    }
    //加入事务
    #[transactional(tx)]
    pub async fn update(&self, dto: SysDept) -> Result<u64> {
        let mut dept = SysDept::from(dto);
        let dept_name = dept.dept_name.clone().unwrap_or_default();
        let parent_id = dept.parent_id.clone().unwrap_or_default();
        let dept_id = dept.dept_id.clone().unwrap_or_default();

        self.check_dept_name_unique(&dept.dept_id, &dept.dept_name, &dept.parent_id)
            .await?;

        if dept_id.eq(&parent_id) {
            return Err(Error::from(format!("修改部门'{dept_name}'失败，上级部门不能是自己")));
        }
        if dept.status.unwrap_or_default().eq(&DEPT_DISABLE)
            && self.select_normal_children_dept_by_id(&dept_id).await? > 0
        {
            return Err(Error::from("该部门包含未停用的子部门！"));
        }
        let new_parent_dept = self.get_dept_by_id(&parent_id).await?;
        let old_dept = self.get_dept_by_id(&dept_id).await?;

        let new_ancestors = format!(
            "{},{}",
            new_parent_dept.ancestors.clone().unwrap_or_default(),
            new_parent_dept.clone().dept_id.unwrap_or_default()
        );
        let old_ancestors = old_dept.ancestors.unwrap_or_default();
        dept.ancestors = Some(new_ancestors.clone());
        self.update_dept_children_tx(&dept_id, &new_ancestors, &old_ancestors, &tx)
            .await?;
        let result = SysDept::update_by_column(&tx, &dept, "dept_id").await?;
        Ok(result.rows_affected)
    }
    #[transactional(tx)]
    pub async fn remove(&self, dept_id: &str) -> Result<u64> {
        if self.has_child_by_dept_id(dept_id).await? {
            return Err(Error::from("存在下级部门,不允许删除"));
        }
        self.check_dept_exist_user(dept_id).await?;
        let res = tx
            .exec(
                "update sys_dept set del_flag = '2' where dept_id = ?",
                vec![to_value!(dept_id)],
            )
            .await?;
        CONTEXT.sys_role_dept_service.remove_by_dept_id_tx(dept_id, &tx).await?;
        Ok(res.rows_affected)
    }
    //根据user id获得本单位及下属单位部门列表 todo
    pub async fn get_dept_tree(&self, user_cache: &UserCache) -> Result<Vec<DeptTreeVO>> {
        let depts = self.list(&DeptQueryDTO::default(), user_cache).await?;
        let depts = depts.into_iter().map(|d| DeptTreeVO::from(d)).collect::<Vec<_>>();
        self.build_dept_tree(depts)
    }
    ///An depts array with a hierarchy
    pub fn build_dept_tree(&self, all_depts: Vec<DeptTreeVO>) -> Result<Vec<DeptTreeVO>> {
        //find tops
        let mut tops = vec![];
        let temp_list = all_depts
            .iter()
            .map(|d| d.id.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        for dept in all_depts.iter() {
            if !temp_list.contains(&dept.parent_id.clone().unwrap_or_default()) {
                tops.push(dept.clone());
            }
        }
        if tops.is_empty() {
            tops = all_depts.clone();
        }
        for mut item in &mut tops {
            self.loop_find_children(&mut item, &all_depts);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_children(&self, arg: &mut DeptTreeVO, all_depts: &Vec<DeptTreeVO>) {
        let mut children = vec![];
        for item in all_depts {
            if item.parent_id == arg.id {
                let mut item = item.clone();
                self.loop_find_children(&mut item, all_depts);
                children.push(item);
            }
        }
        if !children.is_empty() {
            arg.children = Some(children);
        }
    }
    pub async fn select_dept_list_by_role_id(&self, role_id: &str, dept_check_strictly: bool) -> Result<Vec<String>> {
        let ids = sys_dept::select_dept_list_by_role_id(pool!(), role_id, dept_check_strictly)
            .await?
            .iter()
            .map(|d| d.dept_id.clone().unwrap_or_default())
            .collect();
        Ok(ids)
    }
    //查询部门是否存在用户
    pub async fn check_dept_exist_user(&self, dept_id: &str) -> Result<()> {
        let count: u64 = pool!()
            .query_decode(
                "select count(1) from sys_user where dept_id = ? and del_flag = '0'",
                vec![to_value!(dept_id)],
            )
            .await?;
        if count > 0 {
            return Err(Error::from("部门存在用户,不允许删除"));
        }
        Ok(())
    }
    // 是否存在子节点
    pub async fn has_child_by_dept_id(&self, dept_id: &str) -> Result<bool> {
        let count: u64 = pool!()
            .query_decode(
                "select count(1) from sys_dept where del_flag = '0' and parent_id = ?",
                vec![to_value!(dept_id)],
            )
            .await?;
        Ok(count > 0)
    }
    //根据ID查询所有子部门
    async fn select_children_dept_by_id(&self, dept_id: &str) -> Result<Vec<SysDept>> {
        let res: Option<Vec<SysDept>> = pool!()
            .query_decode(
                "select * from sys_dept where find_in_set(?, ancestors)",
                vec![to_value!(dept_id)],
            )
            .await?;
        Ok(res.unwrap_or_default())
    }

    // 根据ID查询所有子部门（正常状态）个数
    pub async fn select_normal_children_dept_by_id(&self, dept_id: &str) -> Result<u64> {
        let count: u64 = pool!()
            .query_decode(
                "select count(*) from sys_dept where status = 0 and del_flag = '0' and find_in_set(?, ancestors)",
                vec![to_value!(dept_id)],
            )
            .await?;
        Ok(count)
    }

    //  校验当前部门名称是否唯一

    pub async fn check_dept_name_unique(
        &self,
        dept_id: &Option<String>,
        dept_name: &Option<String>,
        parent_id: &Option<String>,
    ) -> Result<()> {
        let old_id: Option<String> = pool!()
            .query_decode(
                "select dept_id as count from sys_dept where dept_name=? and parent_id = ? and del_flag = '0'",
                vec![to_value!(dept_name), to_value!(parent_id)],
            )
            .await?;
        if old_id.is_none() || old_id.eq(dept_id) {
            Ok(())
        } else {
            Err(Error::from("部门名称已存在"))
        }
    }

    /**
     * 修改子元素关系
     */
    #[replace_pool]
    async fn update_dept_children(&self, dept_id: &str, new_ancestors: &str, old_ancestors: &str) -> Result<()> {
        let mut children = self.select_children_dept_by_id(dept_id).await?;
        children.iter_mut().for_each(|child| {
            child.ancestors = Some(child.ancestors.clone().unwrap_or_default().replacen(
                old_ancestors,
                new_ancestors,
                1,
            ));
        });
        if children.len() > 0 {
            mapper::sys_dept::update_dept_children(pool!(), children).await?;
        }
        Ok(())
    }

    /**
     * 校验部门是否有数据权限，todo 重写
     *
     * @param dept_id 部门id
     */
    pub async fn check_dept_data_scope(&self, dept_id: &str, user_cache: &crate::UserCache) -> Result<()> {
        if !user_cache.is_admin() {
            let mut dto = DeptQueryDTO::default();
            dto.dept_id = Some(dept_id.to_string());
            let res = self.list(&dto, &user_cache).await?; //todo 重写
            if res.is_empty() {
                return Err(Error::from("没有权限访问部门数据！"));
            }
        }
        Ok(())
    }

    pub(crate) async fn get_dept_by_id(&self, dept_id: &str) -> Result<SysDept> {
        let dept = SysDept::select_by_column(pool!(), field_name!(SysDept.dept_id), dept_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在：{} ！", dept_id)))?;
        Ok(dept)
    }

    pub(crate) async fn get_dept_list_by_ids(&self, dept_ids: &Vec<String>) -> Result<Vec<SysDept>> {
        let res = sys_dept::select_dept_list_by_ids(pool!(), dept_ids).await?;
        Ok(res)
    }
    // export_excel_service!(DeptPageDTO, SysDeptVO,sys_dept::select_page);
}
