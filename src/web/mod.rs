pub mod middleware;
pub use middleware::*;
pub mod data_scope;
pub mod token;
pub mod validator;

pub use token::auth::User;
pub use token::permit::check_permit;

pub const LOGIN_TOKEN_KEY: &'static str = "login_tokens:";
pub fn get_login_user_redis_key(login_user_key: String) -> String {
    format!("{}{}", LOGIN_TOKEN_KEY, login_user_key)
}
