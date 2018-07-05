use super::Result;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct GlobalUserState {
    color: Option<String>,
    display_name: Option<String>,
    emote_sets: Vec<usize>,
    turbo: bool,
    user_id: usize,
}

impl GlobalUserState {
    pub fn parse(tags: HashMap<&str, Option<&str>>) -> Result<GlobalUserState> {
        let emote_sets = if let Some(emote_sets) = get_value!("emote-sets", tags) {
            emote_sets
                .split(",")
                .filter_map(|v| match v.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
                .collect()
        } else {
            Vec::new()
        };
        Ok(GlobalUserState {
            color: get_value_parse!("color", tags),
            display_name: get_value_parse!("display-name", tags),
            emote_sets,
            turbo: get_value!("turbo", tags).unwrap() == "1",
            user_id: get_value_parse!("user-id", tags).unwrap(),
        })
    }
}
