use macros::page_request;
use serde::{Deserialize, Serialize};

#[page_request]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogininforPageDTO {
    pub state: Option<i32>,
}

