#[cfg(feature = "code-gen")]
pub mod gen;
pub mod oa;
pub mod system;

use axum::Router;

pub fn build_api() -> Router {
    let r = Router::new()
        .merge(system::controller::build_auth_api())
        .nest("/system", system::controller::build_system_api())
        .nest("/monitor", system::controller::monitor::build_monitor_api())
        .nest("/common", system::controller::build_common_api())
        .nest("/oa", oa::build_oa_api());

    #[cfg(feature = "code-gen")]
    return Router::new()
        .merge(r)
        .merge(Router::new().nest("/tool", gen::build_gen_api()));

    #[cfg(not(feature = "code-gen"))]
    return r;
}

use crate::error::Error;
use axum::response::{IntoResponse, Response};
use rbatis::Page;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CODE_SUCCESS: u64 = 200;
pub const CODE_FAIL: u64 = 500;

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RespVO<T> {
    pub code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "camelCase")]
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: CODE_SUCCESS,
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: CODE_FAIL,
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: CODE_SUCCESS,
            msg: None,
            data: Some(arg.clone()),
        }
    }
    pub fn from_success_info(msg: &str) -> Self {
        Self {
            code: CODE_SUCCESS,
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn from_error(error: Error) -> Self {
        Self {
            code: 500,
            msg: Some(error.to_string()),
            data: None,
        }
    }
    pub fn from_error_result(code: u64, arg: &Result<T, Error>) -> Self {
        Self {
            code,
            msg: Some(arg.clone().err().unwrap().to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: u64, info: &str) -> Self {
        Self {
            code,
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn judge_result(
        rows_affected: Result<u64, Error>,
        success_msg: &str,
        fail_message: &str,
    ) -> Self {
         match rows_affected {
            Ok(affected) => {
                if affected >= 1 {
                    Self {
                        code: CODE_SUCCESS,
                        msg: Some(success_msg.to_string()),
                        data: None,
                    }
                } else {
                    Self {
                        code: CODE_FAIL,
                        msg: Some(fail_message.to_string()),
                        data: None,
                    }
                }
            }
            Err(err) => {
                Self {
                    code: CODE_FAIL,
                    msg: Some(err.to_string()),
                    data: None,
                }
            }
        }
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T: Serialize + DeserializeOwned> IntoResponse for RespVO<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

/// 自定义输入，serde_json map
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespJson {
    inner: serde_json::map::Map<String, Value>,
}

impl RespJson {
    pub fn new() -> Self {
        Self {
            inner: serde_json::map::Map::new(),
        }
    }
    pub fn success() -> Self {
        let mut inner = serde_json::map::Map::new();
        inner.insert("code".to_string(), CODE_SUCCESS.into());
        Self { inner }
    }
    pub fn success_info(msg: &str) -> Self {
        let mut inner = serde_json::map::Map::new();
        inner.insert("code".to_string(), CODE_SUCCESS.into());
        inner.insert("msg".to_string(), msg.into());
        Self { inner }
    }
    //插入新的
    pub fn insert(&mut self, key: String, v: Value) -> &mut RespJson {
        self.inner.insert(key, v);
        self
    }
}

impl ToString for RespJson {
    fn to_string(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }
}

impl IntoResponse for RespJson {
    fn into_response(self) -> Response {
        axum::Json(self.inner).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageVO<T> {
    pub code: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<T>>,
    pub total: Option<u64>,
    pub msg: Option<String>,
}

impl<T> PageVO<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync,
{
    pub fn from_result(arg: &Result<Page<T>, Error>) -> Self {
        if arg.is_ok() {
            let arg = arg.as_ref().unwrap();
            Self {
                code: CODE_SUCCESS,
                rows: Some(arg.records.clone()),
                total: Some(arg.total),
                msg: None,
            }
        } else {
            Self {
                code: CODE_FAIL,
                rows: None,
                total: None,
                msg: Some(arg.clone().err().unwrap().to_string()),
            }
        }
    }
}

impl<T> ToString for PageVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T: Serialize + DeserializeOwned> IntoResponse for PageVO<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
