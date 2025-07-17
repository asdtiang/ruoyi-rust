use crate::context::CONTEXT;
use crate::{RespJson, RespVO};
use axum::extract::Multipart;
use axum::response::IntoResponse;
use rbatis::rbdc::DateTime;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

//只上传一个
pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
       // let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let date = DateTime::now();
        let new_file_name = format!("{}{}", uuid::Uuid::new_v4().to_string(), file_name);
        let path = PathBuf::from(&CONTEXT.config.upload_path)
            .join("profile")
            .join(date.year().to_string())
            .join(date.mon().to_string())
            .join(&new_file_name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path.as_path(), data).unwrap();
        let mut res = RespJson::success_info("操作成功");
        // res.insert("url".to_string(),json!(path.as_path().display().to_string()));
        res.insert(
            "fileName".to_string(),
            json!(path.display().to_string().replace(&CONTEXT.config.upload_path,"").replace("\\", "/")),
        );
        res.insert("newFileName".to_string(), json!(&new_file_name));
        res.insert("originalFilename".to_string(), json!(file_name));

        return res.into_response();
    }
    RespVO::<u64>::from_error_info(500, "上传失败！").into_response()
}

