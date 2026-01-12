use std::sync::LazyLock;
use crate::system::domain::mapper::sys_logininfor::SysLogininfor;
use rbatis::object_id::ObjectId;
use user_agent_parser::UserAgentParser;

pub static USER_AGENT_PARSER: LazyLock<UserAgentParser> = LazyLock::new(||  UserAgentParser::from_path("./user_agent.yml").unwrap());

pub fn build_logininfor(ip:String,user_agent: String, username: String, status: char, msg: String) -> SysLogininfor {

    let product= USER_AGENT_PARSER.parse_product(&user_agent);
    let browser = format!("{} {}",product.name.unwrap_or_default(),product.major.unwrap_or_default());
    let os=USER_AGENT_PARSER.parse_os(&user_agent);
    let os_str = format!("{} {}",os.name.unwrap_or_default(),os.major.unwrap_or_default());
    SysLogininfor {
        info_id: ObjectId::new().to_string().into(),
        user_name: Some(username),
        ipaddr: Some(ip),
        login_location: None,
        browser: Some(browser),
        os: Some(os_str),
        status: Some(status),
        msg: Some(msg),
        login_time: crate::Now!().into(),
    }
}

// pub(crate) fn timestamp() -> i64 {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
//     ms
// }
