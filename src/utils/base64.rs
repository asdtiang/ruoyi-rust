// Base64字符表
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";


#[derive(Debug, PartialEq)]
pub enum Base64Error {
    InvalidLength,
    InvalidCharacter,
    InvalidPadding,
}

pub struct Base64;

impl Base64 {
    /// 将字节数据编码为Base64字符串
    pub fn encode(data: &[u8]) -> String {

        let mut result = String::new();
        let mut i = 0;
        let len = data.len();

        while i < len {
            // 每次处理3个字节
            let byte1 = data[i];
            let byte2 = if i + 1 < len { data[i + 1] } else { 0 };
            let byte3 = if i + 2 < len { data[i + 2] } else { 0 };

            // 将3个字节（24位）分成4个6位的组
            let group1 = (byte1 >> 2) & 0x3F;
            let group2 = ((byte1 & 0x03) << 4) | ((byte2 >> 4) & 0x0F);
            let group3 = ((byte2 & 0x0F) << 2) | ((byte3 >> 6) & 0x03);
            let group4 = byte3 & 0x3F;

            // 将6位组映射到Base64字符
            result.push(BASE64_CHARS[group1 as usize] as char);
            result.push(BASE64_CHARS[group2 as usize] as char);

            if i + 1 < len {
                result.push(BASE64_CHARS[group3 as usize] as char);
            } else {
                result.push('=');
            }

            if i + 2 < len {
                result.push(BASE64_CHARS[group4 as usize] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }

        result
    }

    /// 将Base64字符串解码为字节数据
    pub fn decode(encoded: &str) -> Result<Vec<u8>, Base64Error> {

        // 移除空白字符和换行符
        let encoded_clean: String = encoded
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let encoded_bytes = encoded_clean.as_bytes();
        let len = encoded_bytes.len();

        // 检查长度是否为4的倍数
        if len % 4 != 0 {
            return Err(Base64Error::InvalidLength);
        }

        let mut result = Vec::new();
        let mut i = 0;

        while i < len {
            // 处理4个Base64字符
            let mut groups = [0u8; 4];
            for j in 0..4 {
                let ch = encoded_bytes[i + j];
                if ch == b'=' {
                    groups[j] = 0;
                } else {
                    groups[j] = match Self::char_to_value(ch as char, BASE64_CHARS) {
                        Some(val) => val,
                        None => return Err(Base64Error::InvalidCharacter),
                    };
                }
            }

            // 将4个6位组重组为3个字节
            let byte1 = (groups[0] << 2) | (groups[1] >> 4);
            let byte2 = (groups[1] << 4) | (groups[2] >> 2);
            let byte3 = (groups[2] << 6) | groups[3];

            result.push(byte1);

            // 检查填充字符以确定需要添加多少个字节
            if encoded_bytes[i + 2] != b'=' {
                result.push(byte2);
            }
            if encoded_bytes[i + 3] != b'=' {
                result.push(byte3);
            }

            i += 4;
        }

        Ok(result)
    }

    /// 将Base64字符转换为对应的6位值
    fn char_to_value(ch: char, charset: &[u8]) -> Option<u8> {
        charset
            .iter()
            .position(|&c| c == ch as u8)
            .map(|pos| pos as u8)
    }

    /// 标准Base64编码
    pub fn encode_standard(data: &[u8]) -> String {
        Self::encode(data)
    }

    /// 标准Base64解码
    pub fn decode_standard(encoded: &str) -> Result<Vec<u8>, Base64Error> {
        Self::decode(encoded)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encode() {
        let data = b"Hello World!";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_basic_decode() {
        let encoded = "SGVsbG8gV29ybGQh";
        let decoded = Base64::decode_standard(encoded).unwrap();
        assert_eq!(decoded, b"Hello World!");
    }

    #[test]
    fn test_padding() {
        let data = b"light wor";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "bGlnaHQgd29y");

        let data = b"light work";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "bGlnaHQgd29yaw==");

        let data = b"light worl";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "bGlnaHQgd29ybA==");
    }

    #[test]
    fn test_empty() {
        let data = b"";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "");

        let decoded = Base64::decode_standard("").unwrap();
        assert_eq!(decoded, b"");
    }

    #[test]
    fn test_single_byte() {
        let data = b"A";
        let encoded = Base64::encode_standard(data);
        assert_eq!(encoded, "QQ==");

        let decoded = Base64::decode_standard("QQ==").unwrap();
        assert_eq!(decoded, b"A");
    }


    #[test]
    fn test_invalid_character() {
        let result = Base64::decode_standard("SGVsbG8gV29ybGQh!");
        assert_eq!(result, Err(Base64Error::InvalidCharacter));
    }

    #[test]
    fn test_invalid_length() {
        let result = Base64::decode_standard("SGVs");
        assert_eq!(result, Err(Base64Error::InvalidLength));
    }

    #[test]
    fn test_with_whitespace() {
        let encoded = "SGVs bG8g V29y bGQh";
        let decoded = Base64::decode_standard(encoded).unwrap();
        assert_eq!(decoded, b"Hello World!");
    }

    #[test]
    fn test_chinese() {
        let data = "你好，世界！".as_bytes();
        let encoded = Base64::encode_standard(data);
        let decoded = Base64::decode_standard(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}

// 使用示例
fn main() {
    println!("=== Base64 编码解码示例 ===");

    // 基本示例
    let text = "Hello, Base64!";
    let data = text.as_bytes();

    println!("原始文本: {}", text);

    // 标准Base64编码
    let encoded = Base64::encode_standard(data);
    println!("Base64编码: {}", encoded);

    // 解码
    let decoded = Base64::decode_standard(&encoded).unwrap();
    let decoded_text = String::from_utf8(decoded).unwrap();
    println!("解码文本: {}", decoded_text);

    println!();

   
}