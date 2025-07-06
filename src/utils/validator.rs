//see https://github.com/soybeanjs/soybean-admin-rust/blob/main/server/core/src/web/validator.rs
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Form, Json,
};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use thiserror::Error;
use validator::{Validate, ValidationErrors};
use crate::RespVO;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid JSON data: {0}")]
    JsonError(String),

    #[error("Invalid form data")]
    FormError,

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Data is missing")]
    DataMissing,
}

#[derive(Debug, Clone)]
pub struct ValidatedForm<T>(pub T);


impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S>,
{
    type Rejection = ValidationError;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let content_type = req
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|value| value.to_str().ok());

            let data = match content_type.as_deref() {
                Some(ct) if ct.contains(mime::APPLICATION_JSON.as_ref()) => {
                    let Json(data) = Json::<T>::from_request(req, state)
                        .await
                        .map_err(|e| ValidationError::JsonError(e.to_string()))?;
                    data
                },
                Some(ct) if ct.contains(mime::APPLICATION_WWW_FORM_URLENCODED.as_ref()) => {
                    let Form(data) = Form::<T>::from_request(req, state)
                        .await
                        .map_err(|_| ValidationError::FormError)?;
                    data
                },
                _ => return Err(ValidationError::DataMissing),
            };

            data.validate().map_err(ValidationError::from)?;
            Ok(ValidatedForm(data))
        }
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ValidationError::JsonError(msg) => (StatusCode::BAD_REQUEST, msg),
            ValidationError::FormError => {
                (StatusCode::BAD_REQUEST, "Invalid form data".to_string())
            },
            ValidationError::Validation(errors) => {
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let messages: Vec<String> = errors
                            .iter()
                            .map(|error| {
                                error
                                    .message
                                    .as_ref()
                                    .map(|cow| cow.to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string())
                            })
                            .collect();
                        messages.join(" ")
                    })
                    .collect();

                (
                    StatusCode::BAD_REQUEST,
                    error_messages.join("\n"),
                )
            },
            ValidationError::DataMissing => {
                (StatusCode::BAD_REQUEST, "Data is missing".to_string())
            },
        };

        RespVO::<u64>::from_error_info(status.as_u16() as u64, &error_message).into_response()
    }
}
