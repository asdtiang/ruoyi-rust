use crate::config::cache_variables::{LOGIN_TOKEN_KEY, REDIS_UUID_CAPTCHA};
use crate::config::global_variables::{ADMIN_NAME, ALL_PERMISSIONS, LOGIN_FAIL, LOGIN_SUC, STATUS_FORBIDDEN};
use crate::context::CONTEXT;
use  crate::system::domain::dto::SignInDTO;
use  crate::system::domain::mapper::sys_user::SysUser;
use  crate::system::domain::mapper::sys_user_role::SysUserRole;
use  crate::system::domain::vo::{JWTToken, SysUserVO, UserCache};
use crate::error::Error;
use crate::pool;
use crate::utils::password_encoder::PasswordEncoder;
use axum::http::HeaderMap;
use rbatis::field_name;
use rbatis::rbdc::DateTime;
use std::time::Duration;
use uuid::Uuid;
const REDIS_KEY_RETRY: &'static str = "login:login_retry";
pub struct SysAuthService {}

impl SysAuthService {
    //返回token
    pub async fn login(&self, arg: &SignInDTO, header_map: &HeaderMap) -> crate::error::Result<String> {
        self.is_need_wait_login_ex().await?;
        let captcha_enabled =CONTEXT.sys_config_service.select_captcha_enabled().await.unwrap_or(false);
        if captcha_enabled {
            if arg.code.is_none() {
                return Err(Error::from("请输入验证码！"));
            }
            let code = arg.code.as_deref().unwrap();
            if code.len() != 4 {
                return Err(Error::from("验证码输入不正确！"));
            }
            let uuid = arg.uuid.as_deref().unwrap_or_default();
            let code_in_cache = CONTEXT
                .cache_service
                .get_string(&format!("{}{}", REDIS_UUID_CAPTCHA, &uuid))
                .await
                .unwrap_or_default();
            if code_in_cache != code {
                return Err(Error::from("验证码输入不正确！"));
            }
        }
        let user: Option<SysUser> =
            SysUser::select_by_column(pool!(), field_name!(SysUser.user_name), &arg.username)
                .await?
                .into_iter()
                .next();
        if user.is_none() {
            return Err(Error::from(format!("账号:{} 不存在!", arg.username)));
        }
        let user = user.unwrap();
        if user.status.eq(&Some(STATUS_FORBIDDEN)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;

        // check pwd
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            &arg.password,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        //todo 加密时间过长，需要换一个，初步定为 https://github.com/RustCrypto/hashes
        //   println!("密码验证{}",Local::now().timestamp_millis()-start);

        if error.is_some() {
            let _ = CONTEXT
                .sys_logininfor_service
                .add_async(&crate::utils::web_utils::build_logininfor(
                    header_map,
                    arg.username.clone(),
                    LOGIN_FAIL,
                    error.clone().unwrap().to_string(),
                ))
                .await;
            self.add_retry_login_limit_num().await?;
            return Err(error.unwrap());
        }
        //   println!("密码验证后{}",Local::now().timestamp_millis()-start);

        let token = self.get_user_info(&user).await;
        //  println!("Token{}",Local::now().timestamp_millis()-start);
        let _ = CONTEXT
            .sys_logininfor_service
            .add_async(&crate::utils::web_utils::build_logininfor(
                header_map,
                arg.username.clone(),
                LOGIN_SUC,
                "成功".to_string(),
            ))
            .await;
        //   println!("写入日志{}",Local::now().timestamp_millis()-start);

        token
    }
    ///is need to wait
    pub async fn is_need_wait_login_ex(&self) -> crate::error::Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT.cache_service.ttl(REDIS_KEY_RETRY).await?;
                if wait_sec > 0 {
                    return Err(Error::from(format!(
                        "操作过于频繁，请等待{}秒后重试!",
                        wait_sec
                    )));
                }
            }
        }
        return Ok(());
    }

    ///Add redis retry record
    pub async fn add_retry_login_limit_num(&self) -> crate::error::Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    REDIS_KEY_RETRY,
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec as u64,
                    )),
                )
                .await?;
        }
        Ok(())
    }
    //fixme 没有用到
     async fn get_user_info_by_token(&self, user_cache: &UserCache) -> crate::error::Result<String> {
        let user = SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), &user_cache.id)
            .await?
            .into_iter()
            .next();
        let user =
            user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", user_cache.user_name)))?;
        self.get_user_info(&user).await
    }
    //返回token
    pub async fn get_user_info(&self, user: &SysUser) -> crate::error::Result<String> {
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let user_id = user
            .user_id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;

        let user_name=user.user_name.clone().unwrap_or_default();
        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;

        let uuid = Uuid::new_v4();

        let user_roles =
            SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), &user_id)
                .await?;
        let role_menu = CONTEXT
            .sys_role_service
            .find_role_menu(&rbatis::table_field_vec!(&user_roles, role_id))
            .await?;
        let menu_ids: Vec<String> = rbatis::table_field_vec!(&role_menu, menu_id)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let dept=CONTEXT.sys_dept_service.detail(user.dept_id.clone().unwrap_or_default().as_str()).await.ok();

        let menus = CONTEXT.sys_menu_service.finds_menu(&menu_ids, &all_menus);
        let permissions: Vec<String> = if ADMIN_NAME.eq(&user_name)
        {
            vec![ALL_PERMISSIONS.to_string()]
        } else {
            rbatis::table_field_vec!(&menus, perms)
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        };

        let mut user:SysUserVO=user.clone().into();
        user.dept=dept;
        let user_cache = UserCache {
            id: user_id.clone(),
            user_name,
            user: Some(user),
            permissions,
            menu_ids,
            roles: CONTEXT
                .sys_user_role_service
                .find_roles_by_user_id(&user_id)
                .await?
                .unwrap(),
            login_time: DateTime::now().set_nano(0),

            token_key: format!("{}{}", LOGIN_TOKEN_KEY, &uuid.to_string()),
        };
        let jwt_token = JWTToken {
            login_user_key: uuid.to_string(),
            exp: DateTime::now().set_nano(0).unix_timestamp_millis() as usize,
        };
        let access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;

        let _ = CONTEXT
            .cache_service
            .set_string_ex(
                &user_cache.token_key,
                &user_cache.to_string(),
                Some(Duration::from_secs(CONTEXT.config.token_expired_min * 60)),
            )
            .await;
        Ok(access_token)
    }

}
