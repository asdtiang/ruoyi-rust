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
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/**
 * 截取字符串
 *
 * @param str 字符串
 * @param start 开始
 * @param end 结束
 * @return 结果
fixme 关注小于0情况
 */
pub fn substring(str: &str, start: usize, end: usize) -> String {
    if str.len() == 0 {
        return str.to_string();
    }
    let mut end = end;
    // if end < 0 {
    //     end = str.len() + end;
    // }
    let mut start = start;
    // if start < 0 {
    //     start = str.len() + start;
    // }

    if end > str.len() {
        end = str.len();
    }

    if start > end {
        return "".to_string();
    }

    // if start < 0 {
    //     start = 0;
    // }
    // if end < 0 {
    //     end = 0;
    // }

    str[start..end].to_string()
}

pub fn substring_between(src_str: &str, open: &str, close: &str) -> String {
    let idx1 = src_str.find(open);
    let idx2 = src_str.find(close);
    if idx1.is_some_and(|idx| idx > 0) && idx2.is_some_and(|idx| idx1.unwrap() < idx) {
        substring(src_str, idx1.unwrap()+1 , idx2.unwrap() )
    } else {
        "".to_string()
    }
}
