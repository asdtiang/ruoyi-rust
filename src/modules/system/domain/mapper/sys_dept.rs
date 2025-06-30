use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql, py_sql};
use rbs::Error;

crud!(SysDept {});
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDept {
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 父部门ID */
    pub parent_id: Option<String>,
    /** 祖级列表 */
    pub ancestors: Option<String>,
    /** 部门名称 */
    pub dept_name: Option<String>,
    /** 显示顺序 */
    pub order_num: Option<u16>,
    /** 负责人 */
    pub leader: Option<String>,
    /** 联系电话 */
    pub phone: Option<String>,
    /** 邮箱 */
    pub email: Option<String>,
    /** 部门状态:0正常,1停用 */
    pub status: Option<char>,
    /** 删除标志（0代表存在 2代表删除） */
    pub del_flag: Option<char>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
}

#[py_sql(
    "`select d.* from sys_dept d where del_flag='0'`
    if dto.dept_id != '':
      ` and (dept_id = #{dto.deptId} or dept_id in ( select t.dept_id from sys_dept t where find_in_set(#{dto.deptId}, ancestors) ))`
    if dto.parent_id != '':
      ` and parent_id = #{dto.parent_id}`
    if dto.deptName != '':
      ` and dept_name like #{'%'+dto.deptName+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if dto.params.dataScope != '':
      `${dto.params.dataScope}`")]
pub async fn select_all_(
    rb: &dyn Executor,
    dto: &crate::system::domain::dto::DeptQueryDTO,
) -> Result<Vec<SysDept>, Error> {
    impled!()
}

#[py_sql(
    "`select d.dept_id  from sys_dept d left join sys_role_dept rd on d.dept_id = rd.dept_id`
` where rd.role_id = #{role_id}`
     if dept_check_strictly:
` and d.dept_id not in (select d.parent_id from sys_dept d inner join sys_role_dept rd on d.dept_id = rd.dept_id and rd.role_id = #{role_id})`
` order by d.parent_id`"
)]
pub async fn select_dept_list_by_role_id(
    rb: &dyn Executor,
    role_id: &str,
    dept_check_strictly: bool,
) -> Result<Vec<SysDept>, Error> {
    impled!()
}

#[html_sql(
    r#"
       `<update id="update_dept_children" parameterType="java.utils.List">`
            `update sys_dept set ancestors =`
            <foreach collection="depts" item="item" index="index"
                separator=" " open="case dept_id " close=" end">
                when #{item.dept_id} then #{item.ancestors}
            </foreach>
            ` where dept_id in`
            <foreach collection="depts" item="item" index="index"
                separator="," open="(" close=")">
               ` #{item.dept_id}`
            </foreach>
        </update>"#
)]
pub async fn update_dept_children(
    rb: &dyn Executor,
    depts: Vec<SysDept>,
) -> Result<Option<u64>, Error> {
    impled!()
}
