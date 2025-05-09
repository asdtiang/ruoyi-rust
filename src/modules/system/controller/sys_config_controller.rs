use axum::extract::Path;
use crate::context::CONTEXT;
use  crate::system::domain::dto::{ConfigAddDTO, ConfigPageDTO, ConfigUpdateDTO};
use  crate::system::domain::vo::SysConfigVO;
use crate::{PageVO, RespVO};
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use macros::pre_authorize;
use rbatis::Page;

#[pre_authorize("system:config:list")]
pub async fn list(dto: Valid<Json<ConfigPageDTO>>) -> impl IntoResponse {
    let dto = dto.0 .0;
    let data = CONTEXT.sys_config_service.page(&dto).await;
    let data = data.map(|d| Page::<SysConfigVO>::from(d));
    PageVO::from_result(&data).into_response()
}

#[pre_authorize("system:config:query")]
pub async fn detail(config_id: Path<String>) -> impl IntoResponse {
    let config = CONTEXT.sys_config_service.detail(&config_id.0).await;
    let config = config.map(|c| SysConfigVO::from(c));
    RespVO::from_result(&config).into_response()
}

#[pre_authorize("system:config:add")]
pub async fn add(dto: Json<ConfigAddDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_config_service.add(dto.0).await;
    RespVO::<u64>::judge_result(&res, "添加成功！", "添加失败！").into_response()
}

#[pre_authorize("system:config:edit")]
pub async fn update(dto: Json<ConfigUpdateDTO>) -> impl IntoResponse {
    let res = CONTEXT.sys_config_service.update(dto.0).await;
    RespVO::from_result(&res).into_response()
}

#[pre_authorize("system:config:remove")]
pub async fn remove(config_id: Path<String>) -> impl IntoResponse {
    let res = CONTEXT.sys_config_service.remove_batch(&config_id).await;
    RespVO::<u64>::judge_result(&res, "", "删除失败！").into_response()
}

#[pre_authorize("system:config:remove")]
pub async fn refresh_cache() -> impl IntoResponse {
    let _ = CONTEXT.sys_config_service.reset_config_cache().await;
    RespVO::<u64>::from_success_info("刷新成功").into_response()
}
