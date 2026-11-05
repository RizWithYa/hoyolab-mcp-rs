# hoyolab-mcp-rs

A fast, single-binary Model Context Protocol (MCP) server for HoYoLAB and HoYoverse games written in Rust. Communicates over standard input/output (stdio JSON-RPC 2.0) with zero runtime dependencies.

Supports Genshin Impact, Honkai: Star Rail, Zenless Zone Zero, and Honkai Impact 3rd.

---

## Features

- **Multi-Game Realtime Notes**: Stamina, Resin, and Battery monitoring with recovery countdowns.
- **Stamina Overflow Alert**: Automated calculation of remaining hours until stamina cap across all bound games.
- **Full Roster & Equipment Details**: In-game characters along with equipped Weapons (Genshin), Light Cones (HSR), and W-Engines (ZZZ).
- **Automated Multi-Game Check-in**: One-call daily web login claim for Genshin, HSR, ZZZ, and Honkai Impact 3rd.
- **Gift Code Redemption**: Official CDKey redemption directly to in-game mail for Genshin, Star Rail, and ZZZ.
- **Dynamic Secret (DS) Generation**: Native MD5 cryptographic header generation supporting Global and Star Rail salts.
- **Zero Configuration Fallback**: Automatic bound UID and server region resolution based on account credentials.

---

## Registered MCP Tools

| Tool | Parameters | Description |
| :--- | :--- | :--- |
| `get_hoyolab_game_roles` | `game_biz` *(optional)* | Fetch all linked game accounts and character UIDs. |
| `get_all_game_records_overview` | none | Retrieve high-level stats across all 4 games (active days, characters, achievements). |
| `get_player_realtime_notes` | `game`, `role_id` *(opt)*, `server` *(opt)* | Real-time stamina/resin, expeditions, commissions, and battery notes. |
| `execute_daily_checkin` | `game` (`all`, `genshin`, `starrail`, `zzz`, `hi3`) | Claim daily check-in rewards on HoYoLAB. |
| `get_player_characters` | `game`, `role_id` *(opt)*, `server` *(opt)* | Full character roster with equipped Weapon / Light Cone / W-Engine. |
| `check_stamina_overflow` | none | Quick status overview and alerts for games approaching maximum stamina. |
| `redeem_gift_code` | `game`, `cdkey`, `role_id` *(opt)*, `server` *(opt)* | Redeem promo codes via the official HoYoverse web exchange API. |

---

## Project Structure

```text
hoyolab-mcp-rs/
├── Cargo.toml                  # Package manifest and dependencies
├── hoyolab.config.yaml         # Local credentials (git-ignored)
├── hoyolab.config.example.yaml # Clean configuration template
└── src/
    ├── main.rs                 # CLI entry point and server startup
    ├── config.rs               # YAML & environment variable loader
    ├── ds.rs                   # Dynamic Secret (DS) & MD5 hash algorithm
    ├── client/
    │   ├── mod.rs              # HTTP client and HoYoLAB API methods
    │   ├── endpoints.rs        # HoYoverse API URL constants
    │   └── models.rs           # Strongly-typed Serde structs
    ├── format/
    │   ├── mod.rs              # Formatter module exports
    │   └── mdx.rs              # Markdown generator for MCP text content
    └── mcp/
        ├── mod.rs              # JSON-RPC dispatcher and stdio event loop
        ├── protocol.rs         # MCP 2024-11-05 protocol definitions
        └── tools.rs            # Tool registration and input schemas
```

---

## Building from Source

### Requirements
- Rust toolchain (1.75+)

### Compilation
```bash
cargo build --release
```

The optimized binary will be created at:
```text
target/release/hoyolab-mcp-rs
```

---

## Configuration

Copy the example configuration file:
```bash
cp hoyolab.config.example.yaml hoyolab.config.yaml
```

Edit `hoyolab.config.yaml`:
```yaml
language: "en-us"
gameId: 6

api:
  timeoutMs: 20000
  minRequestIntervalMs: 100
  concurrency: 4
  maxQueue: 24

auth:
  cookie: "ltoken_v2=...; ltuid_v2=...; account_id_v2=...; cookie_token_v2=...;"
```

### Cookie Guide
- `ltoken_v2` & `ltuid_v2`: Required for daily login and reading player stats/notes.
- `cookie_token_v2` & `account_id_v2`: Required for gift code redemption (`redeem_gift_code`).

#### Quick Cookie Retrieval:
1. Log in to [HoYoLAB](https://www.hoyolab.com) or [HoYoverse Gift Redemption](https://genshin.hoyoverse.com/en/gift).
2. Open DevTools (`F12`) -> **Console**, execute:
   ```javascript
   document.cookie
   ```
3. Copy the output into `hoyolab.config.yaml`.

---

## MCP Client Setup

### OpenCode (`~/.config/opencode/opencode.json`)
```json
{
  "mcp": {
    "hoyolab": {
      "type": "local",
      "command": [
        "/path/to/hoyolab-mcp-rs/target/release/hoyolab-mcp-rs",
        "--config",
        "/path/to/hoyolab-mcp-rs/hoyolab.config.yaml"
      ],
      "enabled": true
    }
  }
}
```

### Claude Desktop (`~/.config/Claude/claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "hoyolab": {
      "command": "/path/to/hoyolab-mcp-rs/target/release/hoyolab-mcp-rs",
      "args": [
        "--config",
        "/path/to/hoyolab-mcp-rs/hoyolab.config.yaml"
      ]
    }
  }
}
```
*(If `--config` is omitted, the binary automatically searches for `hoyolab.config.yaml` in its working directory).*
