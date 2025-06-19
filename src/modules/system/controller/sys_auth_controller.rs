use crate::config::global_constants::{LOGIN_SUC, LOGIN_TOKEN_KEY};
use crate::context::CONTEXT;
use crate::system::domain::dto::SignInDTO;
use crate::system::domain::vo::JWTToken;
use crate::{RespJson, RespVO};
use crate::token_auth::get_token;
use crate::utils::base64::encode;
use crate::web_data::get_user_name;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use macros::pre_authorize;
use std::time::Duration;
use uuid::Uuid;
use crate::modules::system::constants::REDIS_UUID_CAPTCHA;

pub async fn login(header_map: HeaderMap, arg: Json<SignInDTO>) -> impl IntoResponse {
    let token = CONTEXT.sys_auth_service.login(&arg.0, &header_map).await;
    if token.is_err() {
        return RespVO::from_error_result(500, &token).into_response();
    }
    let token = token.unwrap();
    let mut res = RespJson::success();
    res.insert("token".to_string(), token.into());
    res.into_response()
}

pub async fn logout(header_map: HeaderMap) -> impl IntoResponse {
    let token = get_token(&header_map);
    if !token.is_empty() {
        let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if claims.is_ok() {
            let login_user_key = claims.unwrap().login_user_key;
            let _ = CONTEXT
                .sys_logininfor_service
                .add_async(&crate::utils::web_utils::build_logininfor(
                    &header_map,
                    get_user_name(),
                    LOGIN_SUC,
                    "退出成功".to_string(),
                ))
                .await;
            let _ = CONTEXT
                .cache_service
                .del(&format!("{}{}", LOGIN_TOKEN_KEY, login_user_key))
                .await;
        }
    }
    RespVO::<String>::from_success_info("退出成功!").into_response()
}

#[pre_authorize("")]
pub async fn info() -> impl IntoResponse {
    let user_cache = CONTEXT
        .sys_user_service
        .get_user_cache_by_token(&crate::web_data::get_token())
        .await;
    if user_cache.is_err() {
        return RespVO::from_result(&user_cache).into_response();
    }
    let user_cache = user_cache.unwrap();
    let mut res = RespJson::success();
    res.insert(
        "permissions".to_string(),
        serde_json::json!(&user_cache.permissions),
    );
    res.insert("user".to_string(), serde_json::json!(&user_cache.user));
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
    json.insert(
        "captchaEnabled".to_string(),
        //todo 从sysconfig获得
        captcha_enabled.into(),
    );

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
        json.insert("img".to_string(), encode(&png, png.len()).into());
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
