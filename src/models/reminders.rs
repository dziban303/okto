use chrono::Duration;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, RoleId, UserId};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reminder {
    pub minutes: i64,
    #[serde(default)]
    pub channels: Vec<ChannelReminder>,
    #[serde(default)]
    pub users: Vec<UserId>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChannelReminder {
    pub guild: GuildId,
    pub channel: ChannelId,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GuildSettings {
    pub guild: GuildId,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub mentions: Vec<RoleId>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSettings {
    pub user: UserId,
    #[serde(default)]
    pub filters: Vec<String>,
}

impl Reminder {
    pub fn get_duration(&self) -> Duration {
        Duration::minutes(self.minutes)
    }
}
