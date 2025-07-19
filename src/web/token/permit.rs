use crate::config::global_constants::{ADMIN_NAME};
use crate::context::CONTEXT;
use crate::error::Error;
use crate::system::domain::vo::UserCache;
use crate::web::User;
use crate::RespVO;

///Check whether the token_auth is valid and has not expired
pub async fn checked_token(user: &User) -> Result<UserCache, Error> {
    //check token_auth alive

    let key = crate::web::get_login_user_redis_key(user.login_user_key());
    let user_cache: Result<UserCache, Error> = CONTEXT.cache_service.get_json(&key).await;
    match user_cache {
        Ok(u) => {
            //刷新过期时间
            CONTEXT
                .cache_service
                .expire(&key, (CONTEXT.config.token_expired_min * 60) as i32)
                .await?;
            Ok(u)
        }
        Err(e) => Err(e),
    }
}
///Permission to check
/// permit_str支持与非 如sys:user:list||sys:user:delete，暂时不实现，只支持一个权限
pub async fn check_auth(user_cache: &UserCache, permit_str: &str) -> Result<(), Error> {
    let permit_str = permit_str.replace("\"", "");
    if permit_str.len() == 0 {
        return Ok(());
    }
    if user_cache.user_name == ADMIN_NAME {
        return Ok(());
    }

    //let sys_menu = CONTEXT.sys_menu_service.all().await?;
    //权限校验
    for cache_permission in &user_cache.permissions {
        if cache_permission.eq(&permit_str) {
            return Ok(());
        }
    }
    Err(crate::error::Error::from(format!("无权限访问{}", permit_str)))
}

pub async fn check_permit(user: &User, permit_str: &str) -> Option<RespVO<u64>> {
    match checked_token(user).await {
        Ok(data) => {
            match check_auth(&data, permit_str).await {
                Ok(_) => {}
                Err(e) => {
                    //仅提示拦截
                    let resp: RespVO<u64> = RespVO {
                        code: 500,
                        msg: Some(e.to_string()),
                        data: None,
                    };
                    return Some(resp);
                }
            }
        }
        Err(e) => {
            //401 http状态码会强制前端退出当前登陆状态
            let resp: RespVO<u64> = RespVO {
                code: 401,
                msg: Some(format!("Unauthorized for:{}", e.to_string())),
                data: None,
            };
            return Some(resp);
        }
    }
    None
}
