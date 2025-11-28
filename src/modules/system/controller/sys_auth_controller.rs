use crate::config::global_constants::LOGIN_SUC;
use crate::context::CONTEXT;
use crate::system::domain::dto::SignInDTO;
use crate::system::domain::vo::CommonUserVO;
use crate::system::service::REDIS_UUID_CAPTCHA;
use crate::{error_wrapper_unwrap, RespJson, RespVO, UserCache};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use macros::pre_authorize;
use std::time::Duration;
use uuid::Uuid;
use crate::utils::base64::Base64;

pub async fn login(header_map: HeaderMap, arg: Json<SignInDTO>) -> impl IntoResponse {
    error_wrapper_unwrap!(CONTEXT.sys_auth_service.login(&arg.0, &header_map), token);
    let mut res = RespJson::success();
    res.insert("token".to_string(), token.into());
    res.into_response()
}

pub async fn logout(user: Option<axum::Extension<UserCache>>, header_map: HeaderMap) -> impl IntoResponse {
    if let Some(user) = user {
        let user = user.0;
        let _ = CONTEXT
            .sys_logininfor_service
            .add_async(&crate::utils::web_utils::build_logininfor(
                &header_map,
                user.user_name,
                LOGIN_SUC,
                "退出成功".to_string(),
            ))
            .await;
        let _ = CONTEXT
            .cache_service
            .del(&crate::web::get_login_user_redis_key(user.login_user_key))
            .await;
    }
    RespVO::<String>::from_success_info("退出成功!").into_response()
}

#[pre_authorize(user_cache)]
pub async fn info() -> impl IntoResponse {
    let mut res = RespJson::success();
    res.insert("permissions".to_string(), serde_json::json!(&user_cache.permissions));
    error_wrapper_unwrap!(
        CONTEXT.sys_user_service.detail(&user_cache.user_id),
        user
    );
    let user=CommonUserVO::from(user);
   res.insert("user".to_string(), serde_json::json!(&user));
    res.insert(
        "roles".to_string(),
        serde_json::json!(rbatis::table_field_vec!(&user_cache.roles, role_key)),
    );
    res.into_response()
}

pub async fn captcha() -> impl IntoResponse {
    let mut json = RespJson::success();
    let captcha_enabled = CONTEXT
        .sys_config_service
        .select_captcha_enabled()
        .await
        .unwrap_or(false);
    json.insert("captchaEnabled".to_string(), captcha_enabled.into());

    if captcha_enabled {
        let uuid = Uuid::new_v4();
        let (png, code) = make_captcha();
        if CONTEXT.config.debug {
            log::info!("uuid:{},captcha:{}", &uuid.to_string(), &code);
        }
        let result = CONTEXT
            .cache_service
            .set_string_ex(
                &format!("{}{}", REDIS_UUID_CAPTCHA, &uuid.to_string()),
                &code,
                Some(Duration::from_secs(CONTEXT.config.captcha_expired_min * 60)),
            )
            .await;
        if CONTEXT.config.debug == false && result.is_err() {
            return RespVO::from_result(&result).into_response();
        }

        json.insert("uuid".to_string(), uuid.to_string().into());
        json.insert("img".to_string(), Base64::encode(&png).into());
    }
    json.into_response()
}

fn make_captcha() -> (Vec<u8>, String) {
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        // .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let png = captcha.as_png().unwrap();
    let captcha_str = captcha.chars_as_string().to_lowercase();
    (png, captcha_str)
}
