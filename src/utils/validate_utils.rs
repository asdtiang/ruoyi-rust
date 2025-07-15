use std::sync::LazyLock;
use validator::ValidationError;

pub fn string_required(str: &&String) -> Result<(), ValidationError> {
    if str.len() > 0 {
        Ok(())
    } else {
        Err(ValidationError::new("400"))
    }
}
static XSS_REG: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"<(\S*?)[^>]*>.*?|<.*? />").unwrap());
pub fn xss_validator(str: &&String) -> Result<(), ValidationError> {
    if str.len() > 0 {
        if XSS_REG.is_match(str) {
            Err(ValidationError::new("400"))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

//必须须以字母开头，且只能为（小写字母，数字，下滑线）
pub static NORMAL_NAME_REG: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z][a-z0-9_]*$").unwrap()
});