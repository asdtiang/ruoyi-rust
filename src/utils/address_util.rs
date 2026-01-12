use crate::context::CONTEXT;
use crate::error::Error;
use crate::utils::ip_util::is_local_ip;
use std::fmt::Display;
use serde::{Deserialize, Deserializer};

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
#[derive(serde::Deserialize, Debug)]
struct IpData {
    #[serde(deserialize_with = "null_to_default")]
    guo: String,
    #[serde(deserialize_with = "null_to_default")]
    sheng: String,
    #[serde(deserialize_with = "null_to_default")]
    shi: String,
    #[serde(deserialize_with = "null_to_default")]
    qu: String,
    #[serde(deserialize_with = "null_to_default")]
    isp: String,
}

impl Display for IpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{}{}{}{} {}", self.guo, self.sheng, self.shi, self.qu, self.isp)
        )
    }
}
pub async fn get_real_address_by_ip(ip: &str) -> crate::error::Result<String> {
    // 内网不查询
    if is_local_ip(ip) {
        return Ok("内网IP".to_string());
    }
    if CONTEXT.config.address_enabled {
        let id = &CONTEXT.config.apihz_id;
        let key = &CONTEXT.config.apihz_key;
        let body = reqwest::Client::builder()
            .use_rustls_tls() // 显式使用 rustls
            .build()
            .map_err(|e| Error::from(e.to_string()))?
            .get(format!(
                "https://cn.apihz.cn/api/ip/chaapi.php?ip={ip}&id={id}&key={key}",
            ))
            .send()
            .await
            .map_err(|e| Error::from(e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let body = body.trim();
        let mut  value = serde_json::from_str::<IpData>(&body).map_err(|e| Error::from(e.to_string()))?;
        if value.guo .eq("中国"){
            value.guo=String::from("");
        }
        if value.isp .starts_with("中国"){
            value.isp= value.isp.replace("中国","");
        }
        return Ok(value.to_string());
    }
    Ok(String::new())
}
#[tokio::test]
async fn test_get_real_address_by_ip() {
    //let a = get_real_address_by_ip("127.0.0.1").await;
    //println!("a = {:?}", a);
    let a = get_real_address_by_ip("218.86.30.32").await;
    println!("a = {:?}", a);
}
