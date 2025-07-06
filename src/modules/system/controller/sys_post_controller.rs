use crate::config::global_constants::STATUS_NORMAL;
use crate::context::CONTEXT;
use  crate::system::domain::dto::{PostAddDTO, PostPageDTO, PostUpdateDTO};
use  crate::system::domain::mapper::sys_post::SysPost;
use crate::{export_excel_controller, PageVO, RespVO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use macros::pre_authorize;

//#[get("/post/list")]
#[pre_authorize("system:post:list")]
pub async fn list(dto: Json<PostPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_post_service.page(&dto).await;
    PageVO::from_result(&data).into_response()
}


//#[get("/post/{dict_id}")]
#[pre_authorize("system:post:query")]
pub async fn detail(dict_id: Path<String>) -> impl IntoResponse {
    let dict_id = dict_id.0;
    let post_vo = CONTEXT.sys_post_service.detail(&dict_id).await;
    RespVO::from_result(&post_vo).into_response()
}


//#[post("/post")]
#[pre_authorize("system:post:add")]
pub async fn add(arg: crate::ValidatedForm<PostAddDTO>) -> impl IntoResponse {
    let mut data = SysPost::from(arg.0);
    data.create_by = Some(crate::web_data::get_user_name());
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL);
    }
    let rows_affected = CONTEXT.sys_post_service.add(&data).await;
    RespVO::<u64>::judge_result(rows_affected, "", "添加失败！").into_response()
}

//#[put("/post")]
#[pre_authorize("system:post:edit")]
pub async fn update(arg: crate::ValidatedForm<PostUpdateDTO>) -> impl IntoResponse {
    let mut data = SysPost::from(arg.0);
    data.update_by = Some(crate::web_data::get_user_name());
    let rows_affected = CONTEXT.sys_post_service.update(data).await;
    RespVO::<u64>::judge_result(rows_affected, "", "更新失败！").into_response()
}

//#[delete("/post/{post_id}")]
#[pre_authorize("system:post:remove")]
pub async fn remove(dict_id: Path<String>) -> impl IntoResponse {
    let dict_id = dict_id.0;
    let rows_affected = CONTEXT
        .sys_post_service
        .remove_batch(&dict_id)
        .await;
    RespVO::<u64>::judge_result(rows_affected, "", "删除失败！").into_response()
}

export_excel_controller!(
    "system:post:export",
    PostPageDTO,
    CONTEXT,
    sys_post_service,
    export_as_excel_bytes
);

