pub trait IsEmptyString {
    fn is_empty(&self) -> bool;
}

impl IsEmptyString for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => s.is_empty(),
            _ => true,
        }
    }
}

impl IsEmptyString for Option<&str> {
    fn is_empty(&self) -> bool {
        self.is_some_and(|s| s.is_empty()) || self.is_none()
    }
}
/**
 * 截取字符串
 *
 * @param str 字符串
 * @param s_idx 开始
 * @param e_idx 结束
 * @return 结果
 */


pub fn substring(src: &str, s_idx: usize, e_idx: usize) -> String {
    src[s_idx..e_idx].to_string()
}

pub fn substring_between(src: &str, start: &str, end: &str) -> String {
    let s_idx = src.find(start);
    let e_idx = src.find(end);
    if let Some(s_idx) = s_idx {
        if let Some(e_idx) = e_idx {
            if s_idx < e_idx {
                return src[s_idx+ start.len()..e_idx].to_string();
            }
        }
    }
    "".to_string()
}
