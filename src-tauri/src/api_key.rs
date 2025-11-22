use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyStore {
    api_key: Option<String>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        // Try to load from environment variable
        let api_key = env::var("GEMINI_API_KEY").ok();
        Self { api_key }
    }
    
    pub fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }
    
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }
}

pub struct ApiKeyState(pub Mutex<ApiKeyStore>);
