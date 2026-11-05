pub mod protocol;
pub mod tools;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::client::HoYoLabClient;
use crate::format::mdx;
use protocol::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpInitializeResult, McpServerInfo,
    ToolCallResult, ToolCallTextContent, ToolListResult,
};

pub struct McpServer {
    client: HoYoLabClient,
}

impl McpServer {
    pub fn new(client: HoYoLabClient) -> Self {
        Self { client }
    }

    pub async fn run_stdio(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    if let Some(resp_json) = self.handle_request(req).await {
                        stdout.write_all(resp_json.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                }
            }
            line.clear();
        }

        Ok(())
    }

    async fn handle_request(&self, req: JsonRpcRequest) -> Option<String> {
        let id = req.id;

        match req.method.as_str() {
            "initialize" => {
                let res = McpInitializeResult {
                    protocol_version: "2024-11-05".to_string(),
                    capabilities: json!({ "tools": { "listChanged": true } }),
                    server_info: McpServerInfo {
                        name: "hoyolab-mcp-rs".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                    },
                    instructions: "HoYoLAB Rust MCP server: read player stats, daily notes, and manage accounts.".to_string(),
                };
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(res),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "notifications/initialized" => None,
            "tools/list" => {
                let res = ToolListResult {
                    tools: tools::get_tool_definitions(),
                };
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(res),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let (text, is_err) = match self.dispatch_tool(tool_name, args).await {
                    Ok(t) => (t, false),
                    Err(e) => (format!("Error executing {}: {}", tool_name, e), true),
                };

                let res = ToolCallResult {
                    content: vec![ToolCallTextContent {
                        content_type: "text".to_string(),
                        text,
                    }],
                    is_error: if is_err { Some(true) } else { None },
                };

                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(res),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            _ => {
                if id.is_some() {
                    let err = JsonRpcError {
                        code: -32601,
                        message: format!("Method not found: {}", req.method),
                        data: None,
                    };
                    let resp: JsonRpcResponse<Value> = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(err),
                    };
                    serde_json::to_string(&resp).ok()
                } else {
                    None
                }
            }
        }
    }

    async fn resolve_role(&self, game: &str, role_id: Option<&str>, server: Option<&str>) -> Result<(String, String), String> {
        if let (Some(uid), Some(srv)) = (role_id, server) {
            return Ok((uid.to_string(), srv.to_string()));
        }

        let target_biz = match game {
            "genshin" => "hk4e_global",
            "starrail" => "hkrpg_global",
            "zzz" => "nap_global",
            _ => "hk4e_global",
        };

        let roles = self.client.get_user_game_roles(Some(target_biz)).await?;
        let matched = roles.list.into_iter().find(|r| r.game_biz == target_biz);

        if let Some(role) = matched {
            let final_uid = role_id.map(|s| s.to_string()).unwrap_or(role.game_uid);
            let final_server = server.map(|s| s.to_string()).unwrap_or(role.region);
            Ok((final_uid, final_server))
        } else {
            Err(format!("No bound game account found for {}", game))
        }
    }

    async fn dispatch_tool(&self, name: &str, args: Value) -> Result<String, String> {
        match name {
            "get_hoyolab_game_roles" => {
                let game_biz = args.get("game_biz").and_then(|v| v.as_str());
                let data = self.client.get_user_game_roles(game_biz).await?;
                Ok(mdx::format_game_roles(&data))
            }
            "get_all_game_records_overview" => {
                let data = self.client.get_game_record_card(None).await?;
                Ok(mdx::format_game_record_cards(&data))
            }
            "get_player_realtime_notes" => {
                let game = args.get("game").and_then(|v| v.as_str()).unwrap_or("genshin");
                let role_id = args.get("role_id").and_then(|v| v.as_str());
                let server = args.get("server").and_then(|v| v.as_str());

                let (uid, srv) = self.resolve_role(game, role_id, server).await?;

                match game {
                    "genshin" => {
                        let note = self.client.get_genshin_daily_note(&uid, &srv).await?;
                        Ok(mdx::format_genshin_daily_note(&note))
                    }
                    "starrail" => {
                        let note = self.client.get_starrail_daily_note(&uid, &srv).await?;
                        Ok(mdx::format_starrail_daily_note(&note))
                    }
                    "zzz" => {
                        let note = self.client.get_zzz_daily_note(&uid, &srv).await?;
                        Ok(mdx::format_zzz_daily_note(&note))
                    }
                    _ => Err(format!("Unsupported game: {}", game)),
                }
            }
            "execute_daily_checkin" => {
                let game = args.get("game").and_then(|v| v.as_str()).unwrap_or("all");
                let mut results = vec![];

                if game == "all" || game == "genshin" {
                    let res = match self.client.checkin_genshin().await {
                        Ok(s) => format!("- **Genshin Impact**: {}", s),
                        Err(e) => format!("- **Genshin Impact**: {}", e),
                    };
                    results.push(res);
                }
                if game == "all" || game == "starrail" {
                    let res = match self.client.checkin_starrail().await {
                        Ok(s) => format!("- **Honkai: Star Rail**: {}", s),
                        Err(e) => format!("- **Honkai: Star Rail**: {}", e),
                    };
                    results.push(res);
                }
                if game == "all" || game == "zzz" {
                    let res = match self.client.checkin_zzz().await {
                        Ok(s) => format!("- **Zenless Zone Zero**: {}", s),
                        Err(e) => format!("- **Zenless Zone Zero**: {}", e),
                    };
                    results.push(res);
                }
                if game == "all" || game == "hi3" {
                    let res = match self.client.checkin_hi3().await {
                        Ok(s) => format!("- **Honkai Impact 3rd**: {}", s),
                        Err(e) => format!("- **Honkai Impact 3rd**: {}", e),
                    };
                    results.push(res);
                }

                Ok(format!("# Daily Check-in Execution\n\n{}", results.join("\n")))
            }
            "get_player_characters" => {
                let game = args.get("game").and_then(|v| v.as_str()).unwrap_or("starrail");
                let role_id = args.get("role_id").and_then(|v| v.as_str());
                let server = args.get("server").and_then(|v| v.as_str());

                let (uid, srv) = self.resolve_role(game, role_id, server).await?;

                match game {
                    "genshin" => {
                        let data = self.client.get_genshin_characters(&uid, &srv).await?;
                        Ok(mdx::format_genshin_characters(&data))
                    }
                    "starrail" => {
                        let data = self.client.get_starrail_characters(&uid, &srv).await?;
                        Ok(mdx::format_starrail_characters(&data))
                    }
                    "zzz" => {
                        let data = self.client.get_zzz_characters(&uid, &srv).await?;
                        Ok(mdx::format_zzz_characters(&data))
                    }
                    _ => Err(format!("Unsupported game: {}", game)),
                }
            }
            "check_stamina_overflow" => {
                let gi_note = if let Ok((uid, srv)) = self.resolve_role("genshin", None, None).await {
                    self.client.get_genshin_daily_note(&uid, &srv).await.ok()
                } else {
                    None
                };

                let hsr_note = if let Ok((uid, srv)) = self.resolve_role("starrail", None, None).await {
                    self.client.get_starrail_daily_note(&uid, &srv).await.ok()
                } else {
                    None
                };

                let zzz_note = if let Ok((uid, srv)) = self.resolve_role("zzz", None, None).await {
                    self.client.get_zzz_daily_note(&uid, &srv).await.ok()
                } else {
                    None
                };

                Ok(mdx::format_stamina_overview(
                    gi_note.as_ref(),
                    hsr_note.as_ref(),
                    zzz_note.as_ref(),
                ))
            }
            "redeem_gift_code" => {
                let game = args.get("game").and_then(|v| v.as_str()).unwrap_or("genshin");
                let cdkey = args.get("cdkey").and_then(|v| v.as_str()).unwrap_or("");
                if cdkey.trim().is_empty() {
                    return Err("cdkey parameter is required".to_string());
                }

                let role_id = args.get("role_id").and_then(|v| v.as_str());
                let server = args.get("server").and_then(|v| v.as_str());
                let (uid, srv) = self.resolve_role(game, role_id, server).await?;

                self.client.redeem_gift_code(game, &uid, &srv, cdkey.trim()).await
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}
