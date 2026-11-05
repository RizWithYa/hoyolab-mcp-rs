use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub retcode: i64,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameRole {
    pub game_biz: String,
    pub region: String,
    pub game_uid: String,
    pub nickname: String,
    pub level: i64,
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameRolesData {
    pub list: Vec<GameRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardDataItem {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameRecordCard {
    pub game_id: i64,
    pub game_name: String,
    pub nickname: String,
    pub level: i64,
    pub region_name: String,
    #[serde(default)]
    pub data: Vec<CardDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameRecordCardData {
    pub list: Vec<GameRecordCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenshinExpedition {
    pub status: String,
    pub remained_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenshinDailyNoteData {
    pub current_resin: i64,
    pub max_resin: i64,
    pub resin_recovery_time: String,
    pub finished_task_num: i64,
    pub total_task_num: i64,
    pub is_extra_task_reward_received: bool,
    pub current_home_coin: i64,
    pub max_home_coin: i64,
    pub current_expedition_num: i64,
    pub max_expedition_num: i64,
    #[serde(default)]
    pub expeditions: Vec<GenshinExpedition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StarRailDailyNoteData {
    pub current_stamina: i64,
    pub max_stamina: i64,
    pub stamina_recover_time: i64,
    pub current_reserve_stamina: i64,
    pub current_train_score: i64,
    pub max_train_score: i64,
    pub accepted_epedition_num: Option<i64>,
    pub total_expedition_num: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzEnergyProgress {
    pub current: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzEnergy {
    pub progress: ZzzEnergyProgress,
    pub restore: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzVitality {
    pub current: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzVhsSale {
    pub sale_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzDailyNoteData {
    pub energy: ZzzEnergy,
    pub vitality: Option<ZzzVitality>,
    pub vhs_sale: Option<ZzzVhsSale>,
    pub card_sign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenshinWeapon {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub r#type: i64,
    pub rarity: i64,
    pub level: i64,
    pub promote_level: i64,
    pub type_name: String,
    pub desc: String,
    pub affix_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenshinAvatar {
    pub id: i64,
    pub name: String,
    pub element: String,
    pub fetter: i64,
    pub level: i64,
    pub rarity: i64,
    pub actived_constellation_num: i64,
    pub weapon: Option<GenshinWeapon>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenshinIndexData {
    #[serde(default)]
    pub avatars: Vec<GenshinAvatar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StarRailEquip {
    pub id: i64,
    pub level: i64,
    pub rank: i64,
    pub name: String,
    pub rarity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StarRailAvatar {
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub element: String,
    pub rarity: i64,
    pub rank: i64,
    pub equip: Option<StarRailEquip>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StarRailIndexData {
    #[serde(default)]
    pub avatar_list: Vec<StarRailAvatar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzWeapon {
    pub id: i64,
    pub level: i64,
    pub name: String,
    pub star: i64,
    pub rarity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzAvatar {
    pub id: i64,
    pub level: i64,
    #[serde(default)]
    pub name_mi18n: String,
    #[serde(default)]
    pub full_name_mi18n: String,
    #[serde(default)]
    pub camp_name_mi18n: String,
    pub rarity: String,
    pub rank: i64,
    pub weapon: Option<ZzzWeapon>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzIndexData {
    #[serde(default)]
    pub avatar_list: Vec<ZzzAvatar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZzzAvatarInfoData {
    #[serde(default)]
    pub avatar_list: Vec<ZzzAvatar>,
}
