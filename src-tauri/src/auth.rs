use std::collections::HashMap;

static mut USER_MAP: Option<HashMap<String, String>> = None;

/// Initialize authentication
pub fn initialize() {
    unsafe {
        USER_MAP = Some(HashMap::new());
    }

    add_user("user1@email.com", "123");
    add_user("user2@email.com", "456");
}


/// Check if username and password are valid
/// # Arguments
/// * `username`
/// * `password`
pub fn is_valid_user(username: &str, password: &str) -> bool {
    unsafe {
        match &USER_MAP {
            Some(map) => {
                match map.get(username) {
                    Some(pass) => return pass == password,
                    None => return false
                }
            }
            None => return false
        }
    }
}


/// Add username and password
/// # Arguments
/// * `username`
/// * `password`
pub fn add_user(username: &str, password: &str) -> bool {
    unsafe {
        match &mut USER_MAP {
            Some(map) => {
                match map.insert(username.to_string(), password.to_string()) {
                    Some(..) => return true,
                    None => return false
                }
            }
            None => return false
        }
    }
}
