use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub workers: usize,
    pub log_level: String,
    pub req_timeout: u64,
    pub max_retries: u32,
    pub max_body_size_bytes: usize,
    pub user_agent: String,
    pub health_endpoint: String,
    pub storage_path: String,
    pub kvstore_uri: String,
    pub allow_any_origin: bool,
    pub twitter: Option<TwitterConfig>,
    pub origins: Vec<Origin>,
    pub skip_list: Option<Vec<String>>,
    pub allowed_sizes: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterConfig {
    pub cache: CacheConfig,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MediaConfig {
    cache: CacheConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheConfig {
    pub max_age: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Origin {
    pub name: String,
    pub endpoint: String,
    pub cache: CacheConfig,
}
impl AppConfig {
    pub fn validate_origin(&self, origin: &str) -> Option<Origin> {
        self.origins.clone().into_iter().find(|o| o.name == origin)
    }
    pub fn validate_scale(&self, scale: Option<u32>) -> Option<u32> {
        let allowed = self.allowed_sizes.clone().unwrap_or_default();
        if allowed.is_empty() || scale.is_none() {
            Some(scale.unwrap_or(0))
        } else {
            allowed.into_iter().find(|s| s == &scale.unwrap_or(0))
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { max_age: 31536000 }
    }
}
impl Default for Origin {
    fn default() -> Self {
        Self {
            name: String::from("ipfs"),
            endpoint: String::from("https://ipfs.io/ipfs"),
            cache: CacheConfig::default(),
        }
    }
}
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            port: 3030,
            workers: 8,
            req_timeout: 15,
            max_retries: 5,
            skip_list: None,
            max_body_size_bytes: 60000000,
            log_level: "debug".to_string(),
            storage_path: "storage".to_string(),
            kvstore_uri: "http://127.0.0.1:5050".to_string(),
            allowed_sizes: None,
            allow_any_origin: true,
            twitter: None,
            health_endpoint: String::from("/health"),
            user_agent: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            origins: vec![Origin::default()],
        }
    }
}