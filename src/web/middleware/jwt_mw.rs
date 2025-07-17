use crate::context::CONTEXT;
use crate::web::token::auth::{JwtClaims, User};
use axum::http::HeaderMap;
use axum::{body::Body, extract::Request, middleware::Next, response::IntoResponse};

pub const TOKEN_PREFIX: &'static str = "Bearer ";
fn get_token(header_map: &HeaderMap) -> Option<String> {
    header_map.get("authorization").map(|v| {
        let token = v.to_str().unwrap_or_default();
        if token.starts_with(TOKEN_PREFIX) {
            token.replace(TOKEN_PREFIX, "")
        } else {
            return token.to_string();
        }
    })
}
///只抽取token并获得
pub async fn jwt_auth_middleware(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    let token = get_token(req.headers());
    match token {
        None => {}
        Some(t) => {
            let claims = JwtClaims::verify(&CONTEXT.config.jwt_secret, &t);
            if let Ok(c)=claims {
                let user = User::from(c);
                req.extensions_mut().insert(user);
            }
        }
    }
    next.run(req).await.into_response()
}
