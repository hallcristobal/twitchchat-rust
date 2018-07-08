use super::{Badge, Emote, UserNoticeType, UserType};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use super::Result;

#[derive(Eq, PartialEq, Debug)]
pub struct UserNotice {
    badges: Vec<Badge>,
    color: String,
    display_name: Option<String>,
    emotes: Option<Vec<Emote>>,
    id: String,
    login: String,
    moderator: bool,
    notice_type: UserNoticeType,
    room_id: usize,
    subscriber: bool,
    system_msg: String,
    tmi_sent_ts: NaiveDateTime,
    turbo: bool,
    user_id: usize,
    user_type: UserType,
}

impl UserNotice {
    pub fn parse(tags: HashMap<&str, Option<&str>>) -> Result<UserNotice> {
        let badges = if let Some(badges) = get_value!("badges", tags) {
            badges.split(',').map(|v| v.parse().unwrap()).collect()
        } else {
            Vec::new()
        };

        Ok(UserNotice {
            badges,
            color: get_value_parse!("color", tags),
            
        })
    }
}
