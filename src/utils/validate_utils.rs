use std::sync::LazyLock;
use validator::ValidationError;
use zxcvbn::zxcvbn;

pub fn string_required(str: &&String) -> Result<(), ValidationError> {
    if str.len() > 0 {
        Ok(())
    } else {
        Err(ValidationError::new("400"))
    }
}
pub fn password_score(str: &&String) -> Result<(), ValidationError> {
    let estimate = zxcvbn(str, &[]).map_err(|_| ValidationError::new("400"))?;
    if estimate.score() > 2 {
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
pub static NORMAL_NAME_REG: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^[a-z][a-z0-9_]*$").unwrap());

//只能0和1
pub fn status_char(ch: &&char) -> Result<(), ValidationError> {
    let ch = **ch;
    if ch == '0' || ch == '1' {
        Ok(())
    } else {
        Err(ValidationError::new("400"))
    }
}
