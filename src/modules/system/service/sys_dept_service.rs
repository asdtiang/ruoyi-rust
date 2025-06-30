use crate::config::global_constants::{ADMIN_NAME, STATUS_NORMAL};
use crate::context::CONTEXT;
use crate::system::domain::dto::{DeptAddDTO, DeptQueryDTO, DeptUpdateDTO};
use crate::system::domain::mapper;
use crate::system::domain::mapper::sys_dept;
use crate::system::domain::mapper::sys_dept::SysDept;
use crate::system::domain::vo::{DeptTreeVO, SysDeptVO};
use crate::error::Error;
use crate::error::Result;
use crate::web_data::get_user_name;
use crate::{check_unique_sql, export_excel_service, pool};
use macros::data_scope;
use rbatis::field_name;
use rbs::to_value;
use crate::modules::system::constants::{DEPT_DISABLE, DEPT_NORMAL};

pub struct SysDeptService {}

impl SysDeptService {
    #[data_scope(deptAlias = "d", userAlias = "")]
    pub async fn list(&self, arg: &DeptQueryDTO) -> Result<Vec<SysDept>> {
        let data = mapper::sys_dept::select_all_(pool!(), &arg).await?;
        Ok(data)
    }

    pub async fn detail(&self, dept_id: &str) -> Result<SysDeptVO> {
        self.check_dept_data_scope(dept_id).await?;
        let dept = self.get_dept_by_id(dept_id).await?;
        Ok(SysDeptVO::from(dept))
    }

    pub async fn add(&self, dto: DeptAddDTO) -> Result<u64> {
        let mut dept = SysDept::from(dto);
        dept.create_by = Some(get_user_name());
        if dept.status.is_none() {
            dept.status = Some(STATUS_NORMAL);
        }
        self.check_dept_name_unique(
            &dept.dept_id,
            &dept.dept_name.clone().unwrap_or_default(),
            &dept.parent_id.clone().unwrap_or_default(),
        )
        .await?;

        let parent = self.get_dept_by_id(&dept.parent_id.clone().unwrap()).await?;
        if parent.status.unwrap_or_default() != DEPT_NORMAL {
            return Err(Error::from("部门停用，不允许新增"));
        }
        Ok(SysDept::insert(pool!(), &dept).await?.rows_affected)
    }

    pub async fn update(&self, dto: DeptUpdateDTO) -> Result<u64> {
        let mut dept = SysDept::from(dto);
        dept.update_by = Some(crate::web_data::get_user_name());
        let dept_name = dept.dept_name.clone().unwrap_or_default();
        let parent_id = dept.parent_id.clone().unwrap_or_default();
        let dept_id = dept.dept_id.clone().unwrap_or_default();

        self.check_dept_name_unique(&dept.dept_id,&dept_name, &parent_id).await?;

        if dept_id.eq(&parent_id) {
            return Err(Error::from(format!(
                "修改部门'{dept_name}'失败，上级部门不能是自己"
            )));
        }
        if dept.status.unwrap_or_default().eq(&DEPT_DISABLE)
            && self.select_normal_children_dept_by_id(&dept_id).await? > 0
        {
            return Err(Error::from("该部门包含未停用的子部门！"));
        }
        let new_parent_dept = self.get_dept_by_id(&parent_id).await;
        let old_dept = self.get_dept_by_id(&dept_id).await;
        if new_parent_dept.is_ok() && old_dept.is_ok() {
            let new_parent_dept = new_parent_dept?;
            let old_dept = old_dept?;

            let new_ancestors = format!(
                "{},{}",
                new_parent_dept.ancestors.clone().unwrap_or_default(),
                new_parent_dept.clone().dept_id.unwrap_or_default()
            );
            let old_ancestors = old_dept.dept_id.unwrap_or_default();
            dept.ancestors = Some(new_ancestors.clone());
            self.update_dept_children(&dept_id, &new_ancestors, &old_ancestors)
                .await?;
        }
        let result = SysDept::update_by_column(pool!(), &dept, "dept_id").await?;
        Ok(result.rows_affected)
    }

    pub async fn remove(&self, dept_id: &str) -> Result<u64> {
        if self.has_child_by_dept_id(dept_id).await? {
            return Err(Error::from("存在下级部门,不允许删除"));
        }
        self.check_dept_exist_user(dept_id).await?;
        let tx = pool!().acquire_begin().await?;
        let res = pool!()
            .exec(
                "update sys_dept set del_flag = '2' where dept_id = ?",
                vec![to_value!(dept_id)],
            )
            .await?;
        CONTEXT
            .sys_role_dept_service
            .remove_by_dept_id(dept_id)
            .await?;
        tx.commit().await?;
        Ok(res.rows_affected)
    }
    //根据user id获得本单位及下属单位部门列表 todo
    pub async fn get_dept_tree(&self, user_id: &str) -> Result<Vec<DeptTreeVO>> {
        let depts = self.list(&DeptQueryDTO::default()).await?;
        let depts = depts
            .into_iter()
            .map(|d| DeptTreeVO::from(d))
            .collect::<Vec<_>>();
        self.build_dept_tree(&depts)
    }
    ///An depts array with a hierarchy
    pub fn build_dept_tree(&self, all_depts: &Vec<DeptTreeVO>) -> Result<Vec<DeptTreeVO>> {
        //find tops
        let mut tops = all_depts
            .into_iter()
            .filter(|d| d.is_parent())
            .map(|d| d.clone())
            .collect::<Vec<_>>();
        for mut item in &mut tops {
            self.loop_find_children(&mut item, &all_depts);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_children(&self, arg: &mut DeptTreeVO, all_depts: &Vec<DeptTreeVO>) {
        let mut children = vec![];
        for item in all_depts {
            if !item.is_parent() && item.parent_id == arg.id {
                let mut item = item.clone();
                self.loop_find_children(&mut item, all_depts);
                children.push(item);
            }
        }
        if !children.is_empty() {
            arg.children = Some(children);
        }
    }
    pub async fn select_dept_list_by_role_id(
        &self,
        role_id: &str,
        dept_check_strictly: bool,
    ) -> Result<Vec<String>> {
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
    check_unique_sql!(check_dept_name_unique,"sys_dept",dept_id,dept_name,parent_id,"部门名称已存在"
        ,"select count(1) as count from sys_dept where dept_name={?} and parent_id = {?} and del_flag = '0'");

    /**
    * 修改子元素关系

    */
    async fn update_dept_children(
        &self,
        dept_id: &str,
        new_ancestors: &str,
        old_ancestors: &str,
    ) -> Result<()> {
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
    /*
    todo
    <update id="update_dept_children" parameterType="java.utils.List">
            update sys_dept set ancestors =
            <foreach collection="depts" item="item" index="index"
                separator=" " open="case dept_id" close="end">
                when #{item.deptId} then #{item.ancestors}
            </foreach>
            where dept_id in
            <foreach collection="depts" item="item" index="index"
                separator="," open="(" close=")">
                #{item.deptId}
            </foreach>
        </update>

     */

    /*
    <update id="update_dept_children" parameterType="java.utils.List">
         update sys_dept set ancestors =
         <foreach collection="depts" item="item" index="index"
             separator=" " open="case dept_id" close="end">
             when #{item.deptId} then #{item.ancestors}
         </foreach>
         where dept_id in
         <foreach collection="depts" item="item" index="index"
             separator="," open="(" close=")">
             #{item.deptId}
         </foreach>
     </update>

     <update id="updateDeptStatusNormal" parameterType="Long">
          update sys_dept set status = '0' where dept_id in
          <foreach collection="array" item="deptId" open="(" separator="," close=")">
             #{deptId}
         </foreach>
     </update>
     */

    /**
     * 校验部门是否有数据权限
     *
     * @param dept_id 部门id
     */
    pub async fn check_dept_data_scope(&self, dept_id: &str) -> Result<()> {
        if !get_user_name().eq(ADMIN_NAME) {
            let mut dto = DeptQueryDTO::default();
            dto.dept_id = Some(dept_id.to_string());
            let res = self.list(&dto).await?;
            if res.is_empty() {
                return Err(Error::from("没有权限访问部门数据！"));
            }
        }
        Ok(())
    }

    async fn get_dept_by_id(&self, dept_id: &str) -> Result<SysDept> {
        let dept = SysDept::select_by_column(pool!(), field_name!(SysDept.dept_id), dept_id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在：{:?} ！", dept_id)))?;
        Ok(dept)
    }


   // export_excel_service!(DeptPageDTO, SysDeptVO,sys_dept::select_page);
}
