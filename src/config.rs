use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::ds::HoYoLabCookie;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiSettings {
    #[serde(rename = "timeoutMs", default = "default_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(rename = "minRequestIntervalMs", default = "default_interval_ms")]
    pub min_request_interval_ms: u64,
}

fn default_timeout_ms() -> u64 {
    20000
}

fn default_interval_ms() -> u64 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSettings {
    pub cookie: Option<String>,
    pub ltuid: Option<String>,
    pub ltoken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub language: String,
    #[serde(rename = "gameId", default = "default_game_id")]
    pub game_id: u32,
    #[serde(default)]
    pub api: ApiSettings,
    #[serde(default)]
    pub auth: AuthSettings,

    #[serde(skip)]
    pub parsed_cookie: HoYoLabCookie,
}

fn default_game_id() -> u32 {
    2
}

impl AppConfig {
    pub fn load_from_file_or_env<P: AsRef<Path>>(config_path: Option<P>) -> Self {
        let mut config = if let Some(path) = config_path {
            if let Ok(content) = fs::read_to_string(path) {
                serde_yaml::from_str(&content).unwrap_or_default()
            } else {
                Self::default()
            }
        } else if Path::new("hoyolab.config.yaml").exists() {
            let content = fs::read_to_string("hoyolab.config.yaml").unwrap_or_default();
            serde_yaml::from_str(&content).unwrap_or_default()
        } else if Path::new("../HoYoLAB-MCP/hoyolab.config.yaml").exists() {
            let content = fs::read_to_string("../HoYoLAB-MCP/hoyolab.config.yaml").unwrap_or_default();
            serde_yaml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        };

        let env_cookie = std::env::var("HOYOLAB_COOKIE").ok();
        let env_ltuid = std::env::var("HOYOLAB_LTUID").ok();
        let env_ltoken = std::env::var("HOYOLAB_LTOKEN").ok();

        let raw_cookie = config
            .auth
            .cookie
            .clone()
            .or(env_cookie)
            .unwrap_or_else(|| {
                let ltuid = config.auth.ltuid.clone().or(env_ltuid).unwrap_or_default();
                let ltoken = config.auth.ltoken.clone().or(env_ltoken).unwrap_or_default();
                if !ltuid.is_empty() && !ltoken.is_empty() {
                    format!("ltuid_v2={}; ltoken_v2={};", ltuid, ltoken)
                } else {
                    String::new()
                }
            });

        config.parsed_cookie = HoYoLabCookie::parse(&raw_cookie);
        config
    }
}
