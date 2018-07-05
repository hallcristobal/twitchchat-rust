use super::{millis_to_naive_date_time, Result};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Eq, PartialEq, Debug)]
pub struct ClearChat {
    ban_duration: Option<Duration>,
    ban_reason: Option<String>,
    room_id: Option<usize>,
    target_user_id: Option<usize>,
    tmi_sent_ts: Option<NaiveDateTime>,
}

impl ClearChat {
    pub fn parse(tags: HashMap<&str, Option<&str>>) -> Result<ClearChat> {
        let ban_duration = if let Some(&val) = tags.get("ban_duration") {
            if let Some(val) = val {
                Some(Duration::from_secs(val.parse()?))
            } else {
                None
            }
        } else {
            None
        };

        let ts: Option<u64> = get_value_parse!("tmi-sent-ts", tags);
        let tmi_sent_ts = if let Some(ts) = ts {
            millis_to_naive_date_time(ts)
        } else {
            None
        };

        Ok(ClearChat {
            ban_duration,
            ban_reason: get_value_parse!("ban_reason", tags),
            room_id: get_value_parse!("room_id", tags),
            target_user_id: get_value_parse!("target-user-id", tags),
            tmi_sent_ts,
        })
    }
}
