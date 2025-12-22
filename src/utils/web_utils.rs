use crate::system::domain::mapper::sys_logininfor::SysLogininfor;
use axum::http::header::USER_AGENT;
use axum::http::HeaderMap;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use user_agent_parser::UserAgentParser;

pub fn build_logininfor(header_map: &HeaderMap, username: String, status: char, msg: String) -> SysLogininfor {
    let ua_parser = UserAgentParser::from_path("./user_agent.yml").unwrap();
    let user_agent = match header_map.get(USER_AGENT) {
        None => "",
        Some(u) => u.to_str().unwrap_or(""),
    };
    let os = ua_parser.parse_os(user_agent).name.unwrap_or_default().to_string();
    SysLogininfor {
        info_id: ObjectId::new().to_string().into(),
        user_name: Some(username),
        ipaddr: Some("unimpl".to_string()), //fixme
        login_location: None,
        browser: None,
        os: Some(os),
        status: Some(status),
        msg: Some(msg),
        login_time: DateTime::now().set_nano(0).into(),
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
