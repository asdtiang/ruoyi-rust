use crate::config::global_constants::{ADMIN_NAME, LOGIN_FAIL, LOGIN_SUC, LOGIN_TOKEN_KEY, STATUS_FORBIDDEN};
use crate::context::CONTEXT;
use crate::error::{Error, Result};
use crate::modules::system::constants::ALL_PERMISSIONS;
use crate::system::domain::dto::SignInDTO;
use crate::system::domain::mapper::sys_user::SysUser;
use crate::system::domain::mapper::sys_user_role::SysUserRole;
use crate::system::domain::vo::{JwtClaims, SysUserVO, UserCache};
use crate::utils::password_encoder::PasswordEncoder;
use crate::{error_info, pool};
use axum::http::HeaderMap;
use rbatis::field_name;
use rbatis::rbdc::DateTime;
use std::time::Duration;
use uuid::Uuid;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

pub struct SysAuthService {}

impl SysAuthService {
    //返回token
    pub async fn login(&self, sign_in_dto: &SignInDTO, header_map: &HeaderMap) -> Result<String> {
        self.is_need_wait_login_ex(&sign_in_dto.username).await?;
        let captcha_enabled =CONTEXT.sys_config_service.select_captcha_enabled().await.unwrap_or(false);
        if captcha_enabled {
            if sign_in_dto.code.is_none() {
                return Err(Error::from("请输入验证码！"));
            }
            let code = sign_in_dto.code.as_deref().unwrap();
            if code.len() != 4 {
                return Err(Error::from("验证码输入不正确！"));
            }
            let uuid = sign_in_dto.uuid.as_deref().unwrap_or_default();
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
            SysUser::select_by_column(pool!(), field_name!(SysUser.user_name), &sign_in_dto.username)
                .await?
                .into_iter()
                .next();
        if user.is_none() {
            return Err(Error::from(format!("账号:{} 不存在!", sign_in_dto.username)));
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
            &sign_in_dto.password,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        //todo 加密时间过长，需要换一个，初步定为 https://github.com/RustCrypto/hashes

        if error.is_some() {
            let _ = CONTEXT
                .sys_logininfor_service
                .add_async(&crate::utils::web_utils::build_logininfor(
                    header_map,
                    sign_in_dto.username.clone(),
                    LOGIN_FAIL,
                    error.clone().unwrap().to_string(),
                ))
                .await;
            self.add_retry_login_limit_num(&sign_in_dto.username).await?;
            return Err(error.unwrap());
        }

        let token = self.add_to_cache_and_build_token(&user).await;
        let _ = CONTEXT
            .sys_logininfor_service
            .add_async(&crate::utils::web_utils::build_logininfor(
                header_map,
                sign_in_dto.username.clone(),
                LOGIN_SUC,
                "成功".to_string(),
            ))
            .await;

        token
    }
    ///is need to wait
    pub async fn is_need_wait_login_ex(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", REDIS_KEY_RETRY, account))
                .await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT
                    .cache_service
                    .ttl(&format!("{}{}", REDIS_KEY_RETRY, account))
                    .await?;
                if wait_sec > 0 {
                    let mut e = error_info!("req_frequently");
                    e = e.replace("{}", &format!("{}", wait_sec));
                    return Err(Error::from(e));
                }
            }
        }
        Ok(())
    }

    ///Add redis retry record
    pub async fn add_retry_login_limit_num(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", REDIS_KEY_RETRY, account))
                .await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    &format!("{}{}", REDIS_KEY_RETRY, account),
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec,
                    )),
                )
                .await?;
        }
        Ok(())
    }
    // //fixme 没有用到
    //  async fn get_user_info_by_token(&self, user_cache: &UserCache) -> Result<String> {
    //     let user = SysUser::select_by_column(pool!(), field_name!(SysUser.user_id), &user_cache.id)
    //         .await?
    //         .into_iter()
    //         .next();
    //     let user =
    //         user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", user_cache.user_name)))?;
    //     self.get_user_info(&user).await
    // }
    //返回token
    async fn add_to_cache_and_build_token(&self, user: &SysUser) -> Result<String> {
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
        let dept=CONTEXT.sys_dept_service.detail(user.dept_id.clone().unwrap_or_default().as_str(),&user_name).await.ok();

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
            user_name:user_name.clone(),
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
        let jwt_token = JwtClaims {
            login_user_key: uuid.to_string(),
            user_name: user_name.clone(),
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

pub const REDIS_UUID_CAPTCHA: &'static str = "login_captcha:";