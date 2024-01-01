use std::collections::HashMap;

static mut USER_MAP: Option<HashMap<String, String>> = None;
static mut AUTHENTICATED_USER: Option<String> = None;

#[derive(serde::Serialize)]
pub struct AuthInfo {
    is_authenticated: bool,
    user: String
}


/// Initialize authentication
pub fn initialize() {
    unsafe {
        USER_MAP = Some(HashMap::new());
    }

    add_user("user1@email.com", "123");
    add_user("user2@email.com", "456");
}

/// Authenticate
pub fn authenticate(username: &str) -> bool {
    unsafe {
        match &mut AUTHENTICATED_USER {
            Some(..) => return false,
            None => { 
                AUTHENTICATED_USER = Some(username.to_string());
                return true;
            }
        }
    }
}

/// Logout
pub fn logout() {
    unsafe {
        match &mut AUTHENTICATED_USER {
            Some(..) => {
                AUTHENTICATED_USER = None;
                return;
            }
            None => return
        }
    }
}

/// 
pub fn is_authenticated() -> AuthInfo {
    unsafe {
        match &AUTHENTICATED_USER {
            Some(username) => return AuthInfo { is_authenticated: true, user: username.to_string() },
            None => return AuthInfo { is_authenticated: false, user: "".to_string() }
        }
    }
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
