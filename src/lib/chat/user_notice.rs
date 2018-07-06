use super::{Badge, Emote, UserNoticeType, UserType};
use std::time::Instant;

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
    tmi_sent_ts: Instant,
    turbo: bool,
    user_id: usize,
    user_type: UserType,
}
