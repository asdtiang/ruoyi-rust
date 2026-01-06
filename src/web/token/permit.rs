use crate::web::token::auth::UserCache;
use crate::RespVO;

pub async fn check_permit(user_cache: &UserCache, permit_str: &str) -> Option<RespVO<u64>> {
    let permit_str = permit_str.replace("\"", "");
    if permit_str.len() == 0 {
        return None;
    }
    if user_cache.is_admin() {
        return None;
    }
    if user_cache.permissions.contains(&permit_str) {
        return None;
    }
    //仅提示拦截
    let resp: RespVO<u64> = RespVO {
        code: 500,
        msg: Some("无权限访问，请联系管理员".to_string()),
        data: None,
    };
    log::info!("无权限：{}", permit_str);
    Some(resp)
}
///如果没有这个role就返回Some
pub async fn check_role(user_cache: &UserCache, role_str: &str) -> Option<RespVO<u64>> {
    let role_str = role_str.replace("\"", "");
    if role_str.len() == 0 {
        return None;
    }
    if user_cache.is_admin() {
        return None;
    }
    for role in user_cache.roles.clone() {
        if role.role_key.is_some_and(|s| s.eq(&role_str)) {
            return None;
        }
    }
    //todo 查询角色是否禁用
    //仅提示拦截
    let resp: RespVO<u64> = RespVO {
        code: 500,
        msg: Some("无权限访问，请联系管理员".to_string()),
        data: None,
    };
    Some(resp)
}
