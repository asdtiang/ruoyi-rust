use unicode_segmentation::UnicodeSegmentation;

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
 */


pub fn substring_unicode(str: &str, start: usize, end: usize) -> String {
    str.graphemes(true).skip(start).take(end - start).collect()
}

pub fn substring_between(src_str: &str, open: &str, close: &str) -> String {
    let idx1 = src_str.find(open);
    let idx2 = src_str.find(close);
    if idx1.is_some_and(|idx| idx > 0) && idx2.is_some_and(|idx| idx1.unwrap() < idx) {
        substring_unicode(src_str, idx1.unwrap()+1, idx2.unwrap() )
    } else {
        "".to_string()
    }
}
