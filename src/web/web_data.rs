use std::cell::RefCell;


//线程安全的变量，方便会话调用
thread_local!(static LOGIN_USER_KEY: RefCell<String> = RefCell::new("".to_string()););




pub fn set_login_user_key(login_user_key: String) {
    LOGIN_USER_KEY.with(|f| {
        *f.borrow_mut() = login_user_key;
    });
}

pub fn get_login_user_key() -> String {
    let mut login_user_key = "".to_string();
    LOGIN_USER_KEY.with(|f| {
        login_user_key = f.borrow().to_string()
    });
    login_user_key
}



