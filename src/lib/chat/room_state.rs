use std::collections::HashMap;
use super::{Result};

#[derive(Eq, PartialEq, Debug)]
pub struct RoomState {
    braodcaster_language: Option<String>,
    r9k: Option<bool>,
    slow: Option<bool>,
    subs_only: Option<bool>,
}

impl RoomState {
    pub fn parse(tags: HashMap<&str, Option<&str>>) -> Result<RoomState> {
        Ok(RoomState {
            braodcaster_language: get_value_parse!("broadcaster-language", tags),
            r9k: if let Some(r9k) = get_value!("r9k", tags) { Some(r9k == "1") } else { None },
            slow: if let Some(slow) = get_value!("slow", tags) { Some(slow == "1") } else { None },
            subs_only: if let Some(subs) = get_value!("subs-only", tags) { Some(subs == "1") } else { None },
        })
    }
}
