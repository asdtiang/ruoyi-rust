use crate::config::global_constants::STATUS_NORMAL;
use crate::context::CONTEXT;
use crate::system::domain::dto::{DeptAddDTO, DeptQueryDTO, DeptUpdateDTO};
use crate::system::domain::mapper::sys_dept::SysDept;
use crate::system::domain::vo::SysDeptVO;
use crate::{add_marco, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

#[pre_authorize("system:dept:list")]
pub async fn list(dto: Option<Json<DeptQueryDTO>>) -> impl IntoResponse {
    let dto = match dto {
        None => DeptQueryDTO::default(),
        Some(d) => d.0,
    };
    let rows = CONTEXT.sys_dept_service.list(&dto).await;
    let rows = rows.map(|depts| {
        depts
            .iter()
            .map(|d| SysDeptVO::from(d.clone()))
            .collect::<Vec<SysDeptVO>>()
    });
    RespVO::from_result(&rows).into_response()
}

#[pre_authorize("system:dept:list")]
pub async fn exclude_child(dept_id: Path<String>) -> impl IntoResponse {
    let dept_id = dept_id.0;
    let query: DeptQueryDTO = DeptQueryDTO::default();
    let rows = CONTEXT.sys_dept_service.list(&query).await;

    match rows {
        Ok(vo) => {
            let filter = vo
                .into_iter()
                .filter(|d| {
                    !d.dept_id.clone().unwrap_or_default().eq(&dept_id)
                        && !d
                            .parent_id
                            .clone()
                            .unwrap_or_default()
                            .split(",")
                            .collect::<Vec<_>>()
                            .contains(&dept_id.as_str())
                })
                .map(|d| SysDeptVO::from(d))
                .collect::<Vec<_>>();
            RespVO::from(&filter).into_response()
        }

        Err(e) => RespVO::<u64>::from_error(e).into_response(),
    }
}

#[pre_authorize("system:dept:query", user)]
pub async fn detail(dept_id: Path<String>) -> impl IntoResponse {
    let dept_vo = CONTEXT.sys_dept_service.detail(&dept_id.0, &user.user_name).await;
    RespVO::from_result(&dept_vo).into_response()
}

#[pre_authorize("system:dept:add", user)]
pub async fn add(dto: crate::ValidatedForm<DeptAddDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user, SysDept);
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let res = CONTEXT.sys_dept_service.add(data).await;
    RespVO::<u64>::judge_result(res, "", "添加失败！").into_response()
}

#[pre_authorize("system:dept:edit", user)]
pub async fn update(dto: crate::web::validator::ValidatedForm<DeptUpdateDTO>) -> impl IntoResponse {
    add_marco!(data, dto, user, SysDept);
    let res = CONTEXT.sys_dept_service.update(data).await;
    RespVO::<u64>::judge_result(res, "", "更新失败！").into_response()
}

#[pre_authorize("system:dept:remove")]
pub async fn remove(dept_id: Path<String>) -> impl IntoResponse {
    let res = CONTEXT.sys_dept_service.remove(&dept_id.0).await;
    RespVO::<u64>::judge_result(res, "", "删除失败！").into_response()
}
