pub mod endpoints;
pub mod models;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, COOKIE};
use reqwest::Client;
use std::time::Duration;

use crate::config::AppConfig;
use crate::ds::generate_dynamic_secret;
use models::{
    ApiResponse, GameRecordCardData, GameRolesData, GenshinDailyNoteData, GenshinIndexData,
    StarRailDailyNoteData, StarRailIndexData, ZzzAvatarInfoData, ZzzDailyNoteData, ZzzIndexData,
};

#[derive(Clone)]
pub struct HoYoLabClient {
    http: Client,
    pub config: AppConfig,
}

impl HoYoLabClient {
    pub fn new(config: AppConfig) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_millis(config.api.timeout_ms))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { http, config }
    }

    fn build_auth_headers(&self, game: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert("x-rpc-client_type", HeaderValue::from_static("5"));
        headers.insert("x-rpc-app_version", HeaderValue::from_static("1.5.0"));

        let lang = if self.config.language.is_empty() {
            "en-us"
        } else {
            &self.config.language
        };
        if let Ok(val) = HeaderValue::from_str(lang) {
            headers.insert("x-rpc-language", val.clone());
            headers.insert("accept-language", val);
        }

        if let Ok(cookie_val) = HeaderValue::from_str(&self.config.parsed_cookie.raw) {
            headers.insert(COOKIE, cookie_val);
        }

        let ds = generate_dynamic_secret(game, None);
        if let Ok(ds_val) = HeaderValue::from_str(&ds) {
            headers.insert("DS", ds_val);
        }

        headers
    }

    pub async fn get_user_game_roles(&self, game_biz: Option<&str>) -> Result<GameRolesData, String> {
        let mut url = endpoints::TAKUMI_BINDING_API.to_string();
        if let Some(biz) = game_biz {
            url.push_str(&format!("?game_biz={}", biz));
        }

        let headers = self.build_auth_headers(None);
        let resp = self
            .http
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: ApiResponse<GameRolesData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("HoYoLAB API error {}: {}", body.retcode, body.message));
        }

        body.data.ok_or_else(|| "Empty data returned".to_string())
    }

    pub async fn get_game_record_card(&self, hoyolab_uid: Option<&str>) -> Result<GameRecordCardData, String> {
        let uid = hoyolab_uid
            .map(|s| s.to_string())
            .or_else(|| self.config.parsed_cookie.ltuid.clone())
            .ok_or_else(|| "HoYoLAB Account UID is required".to_string())?;

        let url = format!("{}/game_record/card/wapi/getGameRecordCard?uid={}", endpoints::COMMUNITY_API_BASE, uid);
        let headers = self.build_auth_headers(None);
        let resp = self
            .http
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: ApiResponse<GameRecordCardData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("HoYoLAB API error {}: {}", body.retcode, body.message));
        }

        body.data.ok_or_else(|| "Empty card data returned".to_string())
    }

    pub async fn get_genshin_daily_note(&self, role_id: &str, server: &str) -> Result<GenshinDailyNoteData, String> {
        let url = format!(
            "{}/game_record/app/genshin/api/dailyNote?role_id={}&server={}",
            endpoints::COMMUNITY_API_BASE,
            role_id,
            server
        );

        let headers = self.build_auth_headers(Some("genshin"));
        let resp = self
            .http
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: ApiResponse<GenshinDailyNoteData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("HoYoLAB API error {}: {}", body.retcode, body.message));
        }

        body.data.ok_or_else(|| "Empty Genshin daily note returned".to_string())
    }

    pub async fn get_starrail_daily_note(&self, role_id: &str, server: &str) -> Result<StarRailDailyNoteData, String> {
        let url = format!(
            "{}/game_record/hkrpg/api/note?role_id={}&server={}",
            endpoints::COMMUNITY_API_BASE,
            role_id,
            server
        );

        let headers = self.build_auth_headers(Some("starrail"));
        let resp = self
            .http
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: ApiResponse<StarRailDailyNoteData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("HoYoLAB API error {}: {}", body.retcode, body.message));
        }

        body.data.ok_or_else(|| "Empty Star Rail daily note returned".to_string())
    }

    pub async fn get_zzz_daily_note(&self, role_id: &str, server: &str) -> Result<ZzzDailyNoteData, String> {
        let url = format!(
            "{}?role_id={}&server={}",
            endpoints::ZZZ_DAILY_NOTE_API,
            role_id,
            server
        );

        let headers = self.build_auth_headers(Some("zzz"));
        let resp = self
            .http
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: ApiResponse<ZzzDailyNoteData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("HoYoLAB API error {}: {}", body.retcode, body.message));
        }

        body.data.ok_or_else(|| "Empty ZZZ daily note returned".to_string())
    }

    pub async fn checkin_genshin(&self) -> Result<String, String> {
        let headers = self.build_auth_headers(Some("genshin"));
        let body = serde_json::json!({ "act_id": "e202102251931481" });
        let resp = self
            .http
            .post(endpoints::GENSHIN_SIGN_API)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let code = res["retcode"].as_i64().unwrap_or(-1);
        let msg = res["message"].as_str().unwrap_or("Unknown");
        if code == 0 {
            Ok("Genshin Impact check-in successful!".to_string())
        } else if code == -5003 {
            Ok("Genshin Impact already checked in today.".to_string())
        } else {
            Err(format!("Genshin check-in message: {}", msg))
        }
    }

    pub async fn checkin_starrail(&self) -> Result<String, String> {
        let headers = self.build_auth_headers(Some("starrail"));
        let body = serde_json::json!({ "act_id": "e202303301540311" });
        let resp = self
            .http
            .post(endpoints::HSR_SIGN_API)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let code = res["retcode"].as_i64().unwrap_or(-1);
        let msg = res["message"].as_str().unwrap_or("Unknown");
        if code == 0 {
            Ok("Honkai: Star Rail check-in successful!".to_string())
        } else if code == -5003 {
            Ok("Honkai: Star Rail already checked in today.".to_string())
        } else {
            Err(format!("Star Rail check-in message: {}", msg))
        }
    }

    pub async fn checkin_zzz(&self) -> Result<String, String> {
        let mut headers = self.build_auth_headers(Some("zzz"));
        headers.insert("x-rpc-signgame", HeaderValue::from_static("zzz"));
        let body = serde_json::json!({ "act_id": "e202406031448091" });
        let resp = self
            .http
            .post(endpoints::ZZZ_SIGN_API)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let code = res["retcode"].as_i64().unwrap_or(-1);
        let msg = res["message"].as_str().unwrap_or("Unknown");
        if code == 0 {
            Ok("Zenless Zone Zero check-in successful!".to_string())
        } else if code == -5003 {
            Ok("Zenless Zone Zero already checked in today.".to_string())
        } else {
            Err(format!("ZZZ check-in message: {}", msg))
        }
    }

    pub async fn checkin_hi3(&self) -> Result<String, String> {
        let headers = self.build_auth_headers(None);
        let body = serde_json::json!({ "act_id": "e202110291205111" });
        let resp = self
            .http
            .post(endpoints::HI3_SIGN_API)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let res: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let code = res["retcode"].as_i64().unwrap_or(-1);
        let msg = res["message"].as_str().unwrap_or("Unknown");
        if code == 0 {
            Ok("Honkai Impact 3rd check-in successful!".to_string())
        } else if code == -5003 {
            Ok("Honkai Impact 3rd already checked in today.".to_string())
        } else {
            Err(format!("HI3 check-in message: {}", msg))
        }
    }

    pub async fn get_genshin_characters(&self, role_id: &str, server: &str) -> Result<GenshinIndexData, String> {
        let url = format!("{}?role_id={}&server={}", endpoints::GENSHIN_INDEX_API, role_id, server);
        let headers = self.build_auth_headers(Some("genshin"));
        let resp = self.http.get(&url).headers(headers).send().await.map_err(|e| e.to_string())?;
        let body: ApiResponse<GenshinIndexData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("Genshin index API error {}: {}", body.retcode, body.message));
        }
        body.data.ok_or_else(|| "Empty Genshin index data".to_string())
    }

    pub async fn get_starrail_characters(&self, role_id: &str, server: &str) -> Result<StarRailIndexData, String> {
        let url = format!("{}?role_id={}&server={}", endpoints::HSR_INDEX_API, role_id, server);
        let headers = self.build_auth_headers(Some("starrail"));
        let resp = self.http.get(&url).headers(headers).send().await.map_err(|e| e.to_string())?;
        let body: ApiResponse<StarRailIndexData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("Star Rail index API error {}: {}", body.retcode, body.message));
        }
        body.data.ok_or_else(|| "Empty Star Rail index data".to_string())
    }

    pub async fn get_zzz_characters(&self, role_id: &str, server: &str) -> Result<ZzzIndexData, String> {
        let url = format!("{}?role_id={}&server={}", endpoints::ZZZ_INDEX_API, role_id, server);
        let headers = self.build_auth_headers(Some("zzz"));
        let resp = self.http.get(&url).headers(headers).send().await.map_err(|e| e.to_string())?;
        let body: ApiResponse<ZzzIndexData> = resp.json().await.map_err(|e| e.to_string())?;
        if body.retcode != 0 {
            return Err(format!("ZZZ index API error {}: {}", body.retcode, body.message));
        }

        let mut data = body.data.ok_or_else(|| "Empty ZZZ index data".to_string())?;
        for avatar in &mut data.avatar_list {
            let detail_url = format!(
                "{}?role_id={}&server={}&id_list[]={}",
                endpoints::ZZZ_AVATAR_INFO_API,
                role_id,
                server,
                avatar.id
            );
            let detail_headers = self.build_auth_headers(Some("zzz"));
            if let Ok(detail_resp) = self.http.get(&detail_url).headers(detail_headers).send().await {
                if let Ok(detail_body) = detail_resp.json::<ApiResponse<ZzzAvatarInfoData>>().await {
                    if let Some(detail_data) = detail_body.data {
                        if let Some(first) = detail_data.avatar_list.into_iter().next() {
                            avatar.weapon = first.weapon;
                        }
                    }
                }
            }
        }

        Ok(data)
    }

    pub async fn redeem_gift_code(&self, game: &str, role_id: &str, server: &str, cdkey: &str) -> Result<String, String> {
        let endpoint = match game {
            "genshin" => endpoints::GENSHIN_REDEEM_API,
            "starrail" => endpoints::HSR_REDEEM_API,
            "zzz" => endpoints::ZZZ_REDEEM_API,
            _ => return Err(format!("Redeem not supported for game: {}", game)),
        };

        let url = format!(
            "{}?uid={}&region={}&cdkey={}&game_biz={}&lang=en",
            endpoint,
            role_id,
            server,
            cdkey,
            match game {
                "genshin" => "hk4e_global",
                "starrail" => "hkrpg_global",
                "zzz" => "nap_global",
                _ => "",
            }
        );

        let headers = self.build_auth_headers(Some(game));
        let resp = self.http.get(&url).headers(headers).send().await.map_err(|e| e.to_string())?;
        let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let retcode = body["retcode"].as_i64().unwrap_or(-1);
        let msg = body["message"].as_str().unwrap_or("Unknown response");

        if retcode == 0 {
            Ok(format!("Redeem code '{}' success: {}", cdkey, msg))
        } else {
            Err(format!("Redeem code failed (code {}): {}", retcode, msg))
        }
    }
}
