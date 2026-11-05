use md5::{Digest, Md5};
use rand::Rng;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HOYOLAB_GLOBAL_DS_SALT: &str = "6s25p5ox5x14v17754ao5stzg1fnvdin";
pub const HOYOLAB_HSR_DS_SALT: &str = "6s25p5ox5y14umn1p61aqyyvbvvl3lrt";

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_dynamic_secret(game: Option<&str>, custom_salt: Option<&str>) -> String {
    let salt = match custom_salt {
        Some(s) => s,
        None => match game {
            Some("starrail") | Some("hkrpg") => HOYOLAB_HSR_DS_SALT,
            _ => HOYOLAB_GLOBAL_DS_SALT,
        },
    };

    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let r = generate_random_string(6);

    let mut hasher = Md5::new();
    hasher.update(format!("salt={}&t={}&r={}", salt, t, r));
    let hash = format!("{:x}", hasher.finalize());

    format!("{},{},{}", t, r, hash)
}

#[derive(Debug, Clone, Default)]
pub struct HoYoLabCookie {
    pub raw: String,
    pub cookies: HashMap<String, String>,
    pub ltuid: Option<String>,
    pub ltoken: Option<String>,
}

impl HoYoLabCookie {
    pub fn parse(raw: &str) -> Self {
        let mut cookies = HashMap::new();
        for part in raw.split(';') {
            let trimmed = part.trim();
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim().to_string();
                let val = v.trim().to_string();
                if !key.is_empty() {
                    cookies.insert(key, val);
                }
            }
        }

        let ltuid = cookies
            .get("ltuid_v2")
            .or_else(|| cookies.get("ltuid"))
            .or_else(|| cookies.get("account_id_v2"))
            .or_else(|| cookies.get("account_id"))
            .cloned();

        let ltoken = cookies
            .get("ltoken_v2")
            .or_else(|| cookies.get("ltoken"))
            .or_else(|| cookies.get("cookie_token_v2"))
            .or_else(|| cookies.get("cookie_token"))
            .cloned();

        let formatted = cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ");

        Self {
            raw: formatted,
            cookies,
            ltuid,
            ltoken,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ds_format() {
        let ds = generate_dynamic_secret(None, None);
        let parts: Vec<&str> = ds.split(',').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[1].len(), 6);
        assert_eq!(parts[2].len(), 32);
    }

    #[test]
    fn test_cookie_parser() {
        let raw = "ltuid_v2=12345; ltoken_v2=token_abc; mi18nLang=en-us;";
        let parsed = HoYoLabCookie::parse(raw);
        assert_eq!(parsed.ltuid.as_deref(), Some("12345"));
        assert_eq!(parsed.ltoken.as_deref(), Some("token_abc"));
        assert_eq!(parsed.cookies.get("mi18nLang").map(|s| s.as_str()), Some("en-us"));
    }
}
