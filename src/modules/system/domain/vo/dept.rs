use crate::system::domain::mapper::sys_dept::SysDept;
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(macros::Export)]
pub struct SysDeptVO {
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

impl From<SysDept> for SysDeptVO {
    fn from(arg: SysDept) -> Self {
        Self {
            dept_id: arg.dept_id,
            parent_id: arg.parent_id,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader,
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptTreeVO {
    pub id: Option<String>,
    /** 父部门ID */
    #[serde(skip_serializing)]
    pub parent_id: Option<String>,
    pub label: Option<String>,
    /** 子部门 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptTreeVO>>,
}

impl From<SysDept> for DeptTreeVO {
    fn from(arg: SysDept) -> Self {
        Self {
            id: arg.dept_id,
            parent_id: arg.parent_id,
            label: arg.dept_name,
            children: None,
        }
    }
}
impl DeptTreeVO {
    pub fn is_parent(&self) -> bool {
        self.parent_id.is_none() || self.parent_id.clone().unwrap_or_default().eq("0")
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonDeptVO {
    /** 部门ID */
    pub dept_id: Option<String>,
    /** 部门名称 */
    pub dept_name: Option<String>,
    pub leader: Option<String>,
}

impl From<SysDept> for CommonDeptVO {
    fn from(arg: SysDept) -> Self {
        Self {
            dept_id: arg.dept_id,
            dept_name: arg.dept_name,
            leader: arg.leader,
        }
    }
}
