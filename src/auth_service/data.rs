use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::User;

pub static User: Lazy<Mutex<HashMap<u32, User>>> = Lazy::new(|| Mutex::new(None));

fn initialize_from_json(json: &str) {
    let user_data: UserData = serde_json::from_str(json).expect("Invalid JSON");

    let mut global_data = GLOBAL_USER.lock().unwrap();
    *global_data = Some(user_data);
}
