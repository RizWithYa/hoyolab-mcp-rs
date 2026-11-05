use crate::client::models::{
    GameRecordCardData, GameRolesData, GenshinDailyNoteData, GenshinIndexData,
    StarRailDailyNoteData, StarRailIndexData, ZzzDailyNoteData, ZzzIndexData,
};

pub fn format_game_roles(data: &GameRolesData) -> String {
    if data.list.is_empty() {
        return "# HoYoLAB Game Roles\n\nNo bound game roles found.".to_string();
    }

    let mut lines = vec![
        "# HoYoLAB Bound Game Roles".to_string(),
        "".to_string(),
        format!("Found {} bound game role(s):", data.list.len()),
        "".to_string(),
    ];

    for (idx, r) in data.list.iter().enumerate() {
        let name = if r.nickname.is_empty() {
            "Traveler/Trailblazer"
        } else {
            &r.nickname
        };
        lines.push(format!("### {}. {} ({})", idx + 1, name, r.game_biz));
        lines.push(format!("- **UID**: `{}`", r.game_uid));
        lines.push(format!("- **Server / Region**: {}", r.region_name));
        lines.push(format!("- **Level**: Lv. {}", r.level));
        lines.push("".to_string());
    }

    lines.join("\n").trim().to_string()
}

pub fn format_game_record_cards(data: &GameRecordCardData) -> String {
    if data.list.is_empty() {
        return "# HoYoLAB Multi-Game Overview\n\nNo game record cards found.".to_string();
    }

    let mut lines = vec![
        "# HoYoLAB Multi-Game Overview".to_string(),
        "".to_string(),
        format!("Found {} game record(s):", data.list.len()),
        "".to_string(),
    ];

    for (idx, c) in data.list.iter().enumerate() {
        lines.push(format!("## {}. {}", idx + 1, c.game_name));
        lines.push(format!(
            "- **Player**: {} (Lv. {}) · **Server**: {}",
            c.nickname, c.level, c.region_name
        ));
        if !c.data.is_empty() {
            lines.push("- **Key Stats**:".to_string());
            for d in &c.data {
                lines.push(format!("  - **{}**: {}", d.name, d.value));
            }
        }
        lines.push("".to_string());
    }

    lines.join("\n").trim().to_string()
}

pub fn format_genshin_daily_note(data: &GenshinDailyNoteData) -> String {
    let mut lines = vec!["# Real-time Note: GENSHIN IMPACT".to_string(), "".to_string()];

    lines.push(format!(
        "- **Current Resin**: **{}** / {}",
        data.current_resin, data.max_resin
    ));

    let recovery_sec: u64 = data.resin_recovery_time.parse().unwrap_or(0);
    let recovery_str = if recovery_sec == 0 {
        "Fully replenished".to_string()
    } else {
        format!("in ~{:.1} hour(s)", (recovery_sec as f64) / 3600.0)
    };
    lines.push(format!("- **Resin Full Recovery Time**: {}", recovery_str));

    lines.push(format!(
        "- **Daily Commissions**: {} / {} (Extra reward: {})",
        data.finished_task_num,
        data.total_task_num,
        if data.is_extra_task_reward_received {
            "Claimed"
        } else {
            "Not claimed"
        }
    ));

    lines.push(format!(
        "- **Expeditions**: {} / {}",
        data.current_expedition_num, data.max_expedition_num
    ));

    lines.join("\n").trim().to_string()
}

pub fn format_starrail_daily_note(data: &StarRailDailyNoteData) -> String {
    let mut lines = vec!["# Real-time Note: HONKAI: STAR RAIL".to_string(), "".to_string()];

    lines.push(format!(
        "- **Trailblaze Power**: **{}** / {}",
        data.current_stamina, data.max_stamina
    ));

    let recovery_str = if data.stamina_recover_time <= 0 {
        "Fully replenished".to_string()
    } else {
        format!("in ~{:.1} hour(s)", (data.stamina_recover_time as f64) / 3600.0)
    };
    lines.push(format!("- **Full Recovery Time**: {}", recovery_str));

    lines.push(format!(
        "- **Reserved Trailblaze Power**: {} / 2400",
        data.current_reserve_stamina
    ));

    lines.push(format!(
        "- **Daily Training Score**: {} / {}",
        data.current_train_score, data.max_train_score
    ));

    lines.join("\n").trim().to_string()
}

pub fn format_zzz_daily_note(data: &ZzzDailyNoteData) -> String {
    let mut lines = vec!["# Real-time Note: ZENLESS ZONE ZERO".to_string(), "".to_string()];

    lines.push(format!(
        "- **Battery Charge (Energy)**: **{}** / {}",
        data.energy.progress.current, data.energy.progress.max
    ));

    let recovery_str = if data.energy.restore <= 0 {
        "Fully replenished".to_string()
    } else {
        format!("in ~{:.1} hour(s)", (data.energy.restore as f64) / 3600.0)
    };
    lines.push(format!("- **Energy Full Recovery Time**: {}", recovery_str));

    if let Some(vit) = &data.vitality {
        lines.push(format!("- **Engagement / Vitality**: {} / {}", vit.current, vit.max));
    }

    if let Some(vhs) = &data.vhs_sale {
        let state = if vhs.sale_state == "SaleStateDone" {
            "Settled / Ready"
        } else {
            &vhs.sale_state
        };
        lines.push(format!("- **Video Store (VHS Sale)**: {}", state));
    }

    if let Some(card) = &data.card_sign {
        let status = if card == "CardSignNo" {
            "Not scratched yet today"
        } else {
            "Completed"
        };
        lines.push(format!("- **Scratch Card (Newsstand)**: {}", status));
    }

    lines.join("\n").trim().to_string()
}

pub fn format_genshin_characters(data: &GenshinIndexData) -> String {
    let mut lines = vec![
        format!("# Genshin Impact Characters ({})", data.avatars.len()),
        "".to_string(),
    ];

    for a in &data.avatars {
        let weapon_info = match &a.weapon {
            Some(w) => format!("**{}** (Lv. {}, ★{})", w.name, w.level, w.rarity),
            None => "None / Hidden in Privacy Settings".to_string(),
        };
        lines.push(format!(
            "- **{}** (Lv. {}, ★{}, C{}) — *Element:* {} | *Weapon:* {}",
            a.name, a.level, a.rarity, a.actived_constellation_num, a.element, weapon_info
        ));
    }

    lines.join("\n").trim().to_string()
}

pub fn format_starrail_characters(data: &StarRailIndexData) -> String {
    let mut lines = vec![
        format!("# Honkai: Star Rail Characters ({})", data.avatar_list.len()),
        "".to_string(),
    ];

    for a in &data.avatar_list {
        let lc_info = match &a.equip {
            Some(e) => format!("**{}** (Lv. {}, S{}, ★{})", e.name, e.level, e.rank, e.rarity),
            None => "None / Unassigned".to_string(),
        };
        lines.push(format!(
            "- **{}** (Lv. {}, ★{}, E{}) — *Light Cone:* {}",
            a.name, a.level, a.rarity, a.rank, lc_info
        ));
    }

    lines.join("\n").trim().to_string()
}

pub fn format_zzz_characters(data: &ZzzIndexData) -> String {
    let mut lines = vec![
        format!("# Zenless Zone Zero Agents ({})", data.avatar_list.len()),
        "".to_string(),
    ];

    for a in &data.avatar_list {
        let name = if a.name_mi18n.is_empty() {
            &a.full_name_mi18n
        } else {
            &a.name_mi18n
        };
        let weapon_info = match &a.weapon {
            Some(w) => format!("**{}** (Lv. {}, ★{}, Rank {})", w.name, w.level, w.star, w.rarity),
            None => "None / Unassigned".to_string(),
        };
        lines.push(format!(
            "- **{}** (Lv. {}, Rank {}, M{}) — *Camp:* {} | *W-Engine:* {}",
            name, a.level, a.rarity, a.rank, a.camp_name_mi18n, weapon_info
        ));
    }

    lines.join("\n").trim().to_string()
}

pub fn format_stamina_overview(
    gi_note: Option<&GenshinDailyNoteData>,
    hsr_note: Option<&StarRailDailyNoteData>,
    zzz_note: Option<&ZzzDailyNoteData>,
) -> String {
    let mut lines = vec!["# Multi-Game Stamina & Overflow Alert".to_string(), "".to_string()];

    if let Some(gi) = gi_note {
        let hours = gi.resin_recovery_time.parse::<f64>().unwrap_or(0.0) / 3600.0;
        let alert = if gi.current_resin >= gi.max_resin {
            "⚠️ **OVERFLOW / FULL!**"
        } else if hours < 2.0 {
            "⚠️ Full soon (< 2 hours)"
        } else {
            "Normal"
        };
        lines.push("### 1. Genshin Impact (Original Resin)".to_string());
        lines.push(format!("- **Current**: **{}** / {}", gi.current_resin, gi.max_resin));
        lines.push(format!("- **Status**: {}", alert));
        lines.push(format!("- **Full in**: ~{:.1} hour(s)", hours));
        lines.push("".to_string());
    }

    if let Some(hsr) = hsr_note {
        let hours = (hsr.stamina_recover_time as f64) / 3600.0;
        let alert = if hsr.current_stamina >= hsr.max_stamina {
            "⚠️ **OVERFLOW / FULL!**"
        } else if hours < 2.0 {
            "⚠️ Full soon (< 2 hours)"
        } else {
            "Normal"
        };
        lines.push("### 2. Honkai: Star Rail (Trailblaze Power)".to_string());
        lines.push(format!("- **Current**: **{}** / {}", hsr.current_stamina, hsr.max_stamina));
        lines.push(format!("- **Reserved**: {} / 2400", hsr.current_reserve_stamina));
        lines.push(format!("- **Status**: {}", alert));
        lines.push(format!("- **Full in**: ~{:.1} hour(s)", hours));
        lines.push("".to_string());
    }

    if let Some(zzz) = zzz_note {
        let hours = (zzz.energy.restore as f64) / 3600.0;
        let alert = if zzz.energy.progress.current >= zzz.energy.progress.max {
            "⚠️ **OVERFLOW / FULL!**"
        } else if hours < 2.0 {
            "⚠️ Full soon (< 2 hours)"
        } else {
            "Normal"
        };
        lines.push("### 3. Zenless Zone Zero (Battery Charge)".to_string());
        lines.push(format!("- **Current**: **{}** / {}", zzz.energy.progress.current, zzz.energy.progress.max));
        lines.push(format!("- **Status**: {}", alert));
        lines.push(format!("- **Full in**: ~{:.1} hour(s)", hours));
        lines.push("".to_string());
    }

    lines.join("\n").trim().to_string()
}

