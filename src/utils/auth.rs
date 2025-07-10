use crate::RespVO;
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    user_id: String,
    username: String,
    token: String,
    // role: Vec<String>,
    // domain: String,
    // org: Option<String>,
}

impl User {
    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }
}

impl<S> FromRequest<S> for User
where
    S: Send + Sync + 'static,
{
    type Rejection = RespVO<String>;

    fn from_request(
        req: Request,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            req.extensions()
                .get::<User>()
                .cloned()
                .ok_or_else(|| RespVO::from_error_info(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized"))
        }
    }
}
