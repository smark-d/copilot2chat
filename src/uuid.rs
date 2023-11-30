

use chrono::Utc;
use sha2::Sha256;
use sha2::Digest;
use uuid::Uuid;
use std::sync::Once;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref MACHINE_ID: Mutex<String> = Mutex::new(String::new());
    static ref INIT: Once = Once::new();
}

/// Get machine id， only call once
pub fn get_machine_id() -> String {
    INIT.call_once(|| {
        let my_uuid = Uuid::new_v4().to_string();
        let mut hasher = Sha256::new();
        hasher.update(my_uuid.as_bytes());
        let result = hasher.finalize();
        let machine_id = format!("{:x}", result);
        *MACHINE_ID.lock().unwrap() = machine_id;
    });

    MACHINE_ID.lock().unwrap().clone()
}

/// Get request id
pub fn get_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Get session id
pub fn get_session_id() -> String {
        let my_uuid = Uuid::new_v4().to_string();
        let now = Utc::now();
        let timestamp = now.timestamp_millis().to_string();
        my_uuid + &timestamp
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_machine_id() {
        let machine_id = get_machine_id();
        println!("machine_id: {}", machine_id);
        assert_eq!(machine_id.len(), 64);
        assert_eq!(machine_id, get_machine_id());
    }

    #[test]
    fn test_get_request_id() {
        let request_id = get_request_id();
        println!("request_id: {}", request_id);
        assert_eq!(request_id.len(), 36);
    }

    #[test]
    fn test_get_session_id() {
        let session_id = get_session_id();
        println!("session_id: {}", session_id);
        assert_eq!(session_id.len(), 49);
    }
}