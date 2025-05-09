use std::cell::RefCell;


//线程安全的变量，方便会话调用
thread_local!(static USERNAME: RefCell<String> = RefCell::new("".to_string());
static TOKEN: RefCell<String> = RefCell::new("".to_string()));




pub fn set_user_name(user_name: String) {
    USERNAME.with(|f| {
        *f.borrow_mut() = user_name;
    });
}

pub fn get_user_name() -> String {
    let mut user_name = "".to_string();
    USERNAME.with(|f| {
        user_name = f.borrow().to_string()
    });
    user_name
}
pub fn set_token(token: String) {
    TOKEN.with(|f| {
        *f.borrow_mut() = token;
    });
}

pub fn get_token() -> String {
    let mut token = "".to_string();
    TOKEN.with(|f| {
        token = f.borrow().to_string()
    });
    token
}



