use crate::context::CONTEXT;
use crate::error::Error;
use crate::utils::ip_util::is_local_ip;


#[derive(serde::Deserialize, Debug)]
struct IpAddress{
    addr:String
}
pub async fn get_real_address_by_ip(ip: &str) -> crate::error::Result<String> {
    // 内网不查询
    if is_local_ip(ip) {
        return Ok("内网IP".to_string());
    }
    if CONTEXT.config.address_enabled {
        let body = reqwest::get(format!(
            "http://whois.pconline.com.cn/ipJson.jsp?ip={}&json=true",
            ip
        ))
        .await
        .map_err(|e| Error::from(e.to_string()))?
        .text()
        .await
        .map_err(|e| Error::from(e.to_string()))?;
let body=body.trim();
        let value=serde_json::from_str::<IpAddress>(&body);
        return match value {
            Ok(v) => {
                Ok(v.addr)
            }
            Err(e) => {
                Err(Error::from(e.to_string()))
            }
        }
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
    let a = get_real_address_by_ip("127.0.0.1").await;
    println!("a = {:?}", a);
    let a = get_real_address_by_ip("27.158.29.209").await;
    println!("a = {:?}", a);
    let a = get_real_address_by_ip("154.9.241.62").await;
    println!("a = {:?}", a);
}
