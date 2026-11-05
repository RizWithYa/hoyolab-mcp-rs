use serde_json::json;
use crate::mcp::protocol::{ToolDefinition, ToolInputSchema};

pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "get_hoyolab_game_roles".to_string(),
            description: "Fetch all bound game accounts/characters (Genshin, Honkai Star Rail, ZZZ, HI3) under this HoYoLAB account. Requires cookie authentication.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: {
                    let mut m = serde_json::Map::new();
                    m.insert("game_biz".to_string(), json!({
                        "type": "string",
                        "description": "Optional filter: hk4e_global (Genshin), hkrpg_global (Star Rail), nap_global (ZZZ), bh3_global (HI3)"
                    }));
                    m
                },
                required: vec![],
            },
        },
        ToolDefinition {
            name: "get_all_game_records_overview".to_string(),
            description: "Fetch high-level overview cards for all games (Star Rail, ZZZ, Genshin, HI3) in one call. Returns active days, characters unlocked, achievements, and exploration.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: serde_json::Map::new(),
                required: vec![],
            },
        },
        ToolDefinition {
            name: "get_player_realtime_notes".to_string(),
            description: "Fetch real-time stamina/resin, daily commissions, expeditions, and energy for Genshin, Star Rail, or ZZZ. If role_id is omitted, auto-detects your bound account.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: {
                    let mut m = serde_json::Map::new();
                    m.insert("game".to_string(), json!({
                        "type": "string",
                        "enum": ["genshin", "starrail", "zzz"],
                        "description": "Target game (genshin, starrail, zzz)"
                    }));
                    m.insert("role_id".to_string(), json!({
                        "type": "string",
                        "description": "Optional player in-game UID. Auto-detected from bound account if omitted."
                    }));
                    m.insert("server".to_string(), json!({
                        "type": "string",
                        "description": "Optional in-game server (e.g. os_asia, prod_official_asia, prod_gf_jp)"
                    }));
                    m
                },
                required: vec!["game".to_string()],
            },
        },
        ToolDefinition {
            name: "execute_daily_checkin".to_string(),
            description: "Claim official HoYoLAB daily sign-in rewards for Genshin Impact, Honkai: Star Rail, Zenless Zone Zero, and Honkai Impact 3rd.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: {
                    let mut m = serde_json::Map::new();
                    m.insert("game".to_string(), json!({
                        "type": "string",
                        "enum": ["all", "genshin", "starrail", "zzz", "hi3"],
                        "description": "Game to check in: all (default), genshin, starrail, zzz, hi3"
                    }));
                    m
                },
                required: vec![],
            },
        },
        ToolDefinition {
            name: "get_player_characters".to_string(),
            description: "Fetch full in-game characters/agents list with equipped weapons, light cones, and W-engines for Genshin, Star Rail, or ZZZ.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: {
                    let mut m = serde_json::Map::new();
                    m.insert("game".to_string(), json!({
                        "type": "string",
                        "enum": ["genshin", "starrail", "zzz"],
                        "description": "Target game (genshin, starrail, zzz)"
                    }));
                    m.insert("role_id".to_string(), json!({
                        "type": "string",
                        "description": "Optional in-game player UID. Auto-detected from bound account if omitted."
                    }));
                    m.insert("server".to_string(), json!({
                        "type": "string",
                        "description": "Optional server identifier. Auto-detected from bound account if omitted."
                    }));
                    m
                },
                required: vec!["game".to_string()],
            },
        },
        ToolDefinition {
            name: "check_stamina_overflow".to_string(),
            description: "Calculate and summarize stamina / resin / battery overflow alerts across all your bound HoYoLAB games.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: serde_json::Map::new(),
                required: vec![],
            },
        },
        ToolDefinition {
            name: "redeem_gift_code".to_string(),
            description: "Redeem official gift / promotion CDKey codes for Genshin Impact, Honkai: Star Rail, or Zenless Zone Zero.".to_string(),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: {
                    let mut m = serde_json::Map::new();
                    m.insert("game".to_string(), json!({
                        "type": "string",
                        "enum": ["genshin", "starrail", "zzz"],
                        "description": "Target game (genshin, starrail, zzz)"
                    }));
                    m.insert("cdkey".to_string(), json!({
                        "type": "string",
                        "description": "Gift / promo code to redeem (e.g. GENSHINGIFT, HSRGRANDOPEN1)"
                    }));
                    m.insert("role_id".to_string(), json!({
                        "type": "string",
                        "description": "Optional player UID. Auto-detected from bound account if omitted."
                    }));
                    m.insert("server".to_string(), json!({
                        "type": "string",
                        "description": "Optional game server. Auto-detected from bound account if omitted."
                    }));
                    m
                },
                required: vec!["game".to_string(), "cdkey".to_string()],
            },
        },
    ]
}
