use crate::context::CONTEXT;
use crate::error::Error;
use crate::utils::ip_util::is_local_ip;
use std::fmt::Display;

#[derive(serde::Deserialize, Debug)]
struct IpData {
    guo: String,
    sheng: String,
    shi: String,
    qu: String,
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
        let id=&CONTEXT.config.apihz_id;
        let key=&CONTEXT.config.apihz_key;
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
        let value = serde_json::from_str::<IpData>(&body).map_err(|e| Error::from(e.to_string()))?;
        return Ok(value.to_string());
    }
    Ok(String::new())
}

/*
try
{
String rspStr = HttpUtils.sendGet(IP_URL, "ip=" + ip + "&json=true", Constants.GBK);
if (StringUtils.isEmpty(rspStr))
{
log.error("获取地理位置异常 {}", ip);
return UNKNOWN;
}
JSONObject obj = JSON.parseObject(rspStr);
String region = obj.getString("pro");
String city = obj.getString("city");
return String.format("%s %s", region, city);
}
catch (Exception e)
{
log.error("获取地理位置异常 {}", ip);
}
}
return UNKNOWN;
}*/
#[tokio::test]
async fn test_get_real_address_by_ip() {
    //let a = get_real_address_by_ip("127.0.0.1").await;
    //println!("a = {:?}", a);
    let a = get_real_address_by_ip("27.158.29.209").await;
    println!("a = {:?}", a);
}
