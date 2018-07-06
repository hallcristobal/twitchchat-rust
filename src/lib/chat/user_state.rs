use super::{Emote, UserType};

#[derive(Eq, PartialEq, Debug)]
pub struct UserState {
    color: String,
    display_name: Option<String>,
    emote_sets: Option<Vec<Emote>>,
    moderator: bool,
    subscriber: bool,
    turbo: bool,
    user_type: UserType,
}
