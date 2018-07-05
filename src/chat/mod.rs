macro_rules! get_value_parse {
    ($key:expr, $tags:path) => {
        if let Some(&val) = $tags.get($key) {
            if let Some(val) = val {
                Some(val.parse()?)
            } else {
                None
            }
        } else {
            None
        }
    };
}

macro_rules! get_value {
    ($key:expr, $tags:path) => {
        if let Some(&val) = $tags.get($key) {
            if let Some(val) = val {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    };
}

mod badge;
mod clear_chat;
mod command;
mod emote;
mod error;
mod global_user_state;
mod message;
mod priv_msg;
mod raid;
mod room_state;
mod sub;
mod sub_gift;
mod sub_plan;
mod user_notice;
mod user_notice_type;
mod user_state;
mod user_type;

pub use self::badge::Badge;
pub use self::clear_chat::ClearChat;
pub use self::command::Command;
pub use self::emote::Emote;
pub use self::global_user_state::GlobalUserState;
pub use self::message::Message;
pub use self::priv_msg::PrivMsg;
pub use self::raid::Raid;
pub use self::room_state::RoomState;
pub use self::sub::Sub;
pub use self::sub_gift::SubGift;
pub use self::sub_plan::SubPlan;
pub use self::user_notice::UserNotice;
pub use self::user_notice_type::UserNoticeType;
pub use self::user_state::UserState;
pub use self::user_type::UserType;

pub type Result<T> = ::std::result::Result<T, self::error::Error>;

use chrono::NaiveDateTime;
fn millis_to_naive_date_time(millis: u64) -> Option<NaiveDateTime> {
    // I hate this
    let sec: u64 = millis / 1000;
    let nano: u64 = (millis - (sec * 1000)) * 1_000_000;
    NaiveDateTime::from_timestamp_opt(sec as i64, nano as u32)
}
