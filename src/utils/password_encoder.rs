use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        hash(raw_password, DEFAULT_COST).unwrap()
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        verify(raw_password, password).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::utils::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
        assert!(PasswordEncoder::verify(&s, "123456"));
    }

    #[test]
    fn test_verify() {
        let password = "$2a$10$l2MaBPlmK8uYb.X.WfQQUekj0kcaDcLbBGsWF57Dw/LK8yqpGc5mS";
        let raw_password = "admin123";

        assert!(PasswordEncoder::verify(password, raw_password));

        //let encode_password = PasswordEncoder::encode(password);
        //assert!(PasswordEncoder::verify(&encode_password, password));
    }
}
