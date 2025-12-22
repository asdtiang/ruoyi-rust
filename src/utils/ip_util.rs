use axum::http::{HeaderMap, HeaderValue};

fn is_ip_empty(ip: &Option<&HeaderValue>) -> bool {
    ip.is_none_or(|hv| {
        let s = hv.to_str().unwrap().to_lowercase();
        return s.len() == 0 || s.eq("unknown");
    })
}
/**
 * 获取客户端IP
 *
 * @param head HeaderMap
 * @return IP地址
 */
pub fn get_ip_addr(head: &HeaderMap) -> Option<String> {
    let mut ip = head.get("x-forwarded-for");
    if is_ip_empty(&ip) {
        ip = head.get("Proxy-Client-IP");
    }

    if is_ip_empty(&ip) {
        ip = head.get("X-Forwarded-For");
    }
    if is_ip_empty(&ip) {
        ip = head.get("WL-Proxy-Client-IP");
    }
    if is_ip_empty(&ip) {
        ip = head.get("X-Real-IP");
    }

    // if is_ip_empty(&ip)
    // {
    //     ip = req.
    // }
    let ip = ip.map(|hv| hv.to_str().unwrap()).map(|s| {
        if "0:0:0:0:0:0:0:1".eq(s) {
            "127.0.0.1".to_string()
        } else {
            get_multistage_reverse_proxy_ip(s).to_string()
        }
    });
    ip
}
/**
 * 从多级反向代理中获得第一个非unknown IP地址
 *
 * @param ip 获得的IP地址
 * @return 第一个非unknown IP地址
 */
pub fn get_multistage_reverse_proxy_ip(ip: &str) -> String {
    let ip = ip;
    // 多级反向代理检测
    if ip.contains(",") {
        let ips = ip.trim().split(",").collect::<Vec<_>>();
        for sub_ip in ips {
            let sub_ip = sub_ip.trim();
            if sub_ip.len() > 0 && !sub_ip.eq("unknown") {
                return string::substring_unicode(ip, 0, 255);
            }
        }
    }
    "".to_string()
}

use crate::utils::string;
use std::net::{IpAddr, Ipv6Addr};

pub fn is_local_ip(ip: &str) -> bool {
    let ip = ip.parse::<IpAddr>().unwrap();
    match ip {
        IpAddr::V4(ipv4) => {
            // 检查 IPv4 私有地址范围
            ipv4.is_private() || ipv4.is_loopback() || ipv4.is_link_local()
        }
        IpAddr::V6(ipv6) => {
            // 检查 IPv6 私有地址范围
            is_private_ipv6(&ipv6)
        }
    }
}

fn is_private_ipv6(ipv6: &Ipv6Addr) -> bool {
    // 唯一本地地址 (fc00::/7)
    let unique_local = (ipv6.segments()[0] & 0xfe00) == 0xfc00;

    // 链路本地地址 (fe80::/10)
    let link_local = (ipv6.segments()[0] & 0xffc0) == 0xfe80;

    // 环回地址 (::1)
    let loopback = *ipv6 == Ipv6Addr::LOCALHOST;

    unique_local || link_local || loopback
}
