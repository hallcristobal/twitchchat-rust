use super::{millis_to_naive_date_time, Result};
use super::{Badge, Emote, UserType};
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct PrivMsg {
    badges: Vec<Badge>,
    color: Option<String>,
    display_name: Option<String>,
    emotes: Option<Vec<Emote>>,
    id: String,
    moderator: bool,
    room_id: usize,
    subscriber: bool,
    tmi_sent_ts: NaiveDateTime,
    turbo: bool,
    user_id: usize,
    user_type: UserType,
    bits: Option<usize>, // TODO: Parse bits tag
    message: Option<String>,
}

impl PrivMsg {
    pub fn parse(tags: HashMap<&str, Option<&str>>) -> Result<PrivMsg> {
        let ts: Option<u64> = get_value_parse!("tmi-sent-ts", tags);
        let tmi_sent_ts = if let Some(ts) = ts {
            millis_to_naive_date_time(ts)
        } else {
            None
        };

        let badges = if let Some(badges) = get_value!("badges", tags) {
            badges.split(',').map(|v| v.parse().unwrap()).collect()
        } else {
            Vec::new()
        };

        let emotes = if let Some(emotes) = get_value!("emotes", tags) {
            Some(
                emotes
                    .split("/")
                    .filter_map(|v| match v.parse::<Emote>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect(),
            )
        } else {
            None
        };
        Ok(PrivMsg {
            badges,
            color: get_value_parse!("color", tags),
            display_name: get_value_parse!("display-name", tags),
            emotes,
            id: get_value_parse!("id", tags).unwrap(),
            moderator: get_value!("mod", tags).unwrap() == "1",
            room_id: get_value_parse!("room-id", tags).unwrap(),
            subscriber: get_value!("subscriber", tags).unwrap() == "1",
            tmi_sent_ts: tmi_sent_ts.unwrap(),
            turbo: get_value!("turbo", tags).unwrap() == "1",
            user_id: get_value_parse!("user-id", tags).unwrap(),
            user_type: get_value_parse!("user-type", tags).unwrap(),
            bits: get_value_parse!("bits", tags),
            message: get_value_parse!("message", tags),
        })
    }
}
