use std::time::Duration;
use crate::config::global_constants::ADMIN_USERID;
use crate::context::CONTEXT;
use crate::error::Result;
use crate::UserCache;

pub struct SysUserOnlineService {}

impl SysUserOnlineService {
    pub async fn force_logout_by_user_id(&self, user_id: &str) -> Result<bool> {
        let keys = CONTEXT
            .cache_service
            .keys(&crate::web::get_login_user_redis_key("*"))
            .await?;
        for key in keys {
            let user_cache: UserCache = CONTEXT.cache_service.get_json(&key).await?;
            if user_cache.user_id.eq(user_id) {
                return CONTEXT.cache_service.del(&key).await;
            }
        }
        Ok(false)
    }
    pub async fn force_logout_by_token(&self, token: &str) -> Result<bool> {
        CONTEXT
            .cache_service
            .del(&crate::web::get_login_user_redis_key(token))
            .await
    }
    pub async fn get_user_cache_by_token(&self, login_user_key: &str) -> Result<UserCache> {
        CONTEXT
            .cache_service
            .get_json::<UserCache>(&crate::web::get_login_user_redis_key(login_user_key))
            .await
    }

    //更改所有的用户缓存，如果
    pub async fn refresh_all_user_cache(&self) -> Result<bool> {
        let keys = CONTEXT
            .cache_service
            .keys(&crate::web::get_login_user_redis_key("*"))
            .await?;

        for key in keys {
            let mut user_cache: UserCache = CONTEXT.cache_service.get_json(&key).await?;
            let user_id = user_cache.user_id.clone();
            if !user_id.eq(&ADMIN_USERID) {
                let (permissions, menu_ids, roles) =
                    CONTEXT.sys_auth_service.load_menu_role_by_user_id(&user_id).await?;
                user_cache.permissions = permissions;
                user_cache.menu_ids = menu_ids;
                user_cache.roles = roles;
                let _ = CONTEXT
                    .cache_service
                    .set_string_ex(
                        &user_cache.token_key,
                        &user_cache.to_string(),
                        Some(Duration::from_secs(CONTEXT.config.token_expired_min * 60)),
                    )
                    .await;
            }
        }
        Ok(false)
    }
}
