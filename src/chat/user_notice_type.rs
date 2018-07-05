use super::{Raid, Sub, SubGift};

#[derive(Eq, PartialEq, Debug)]
pub enum UserNoticeType {
    Sub(Sub),
    Resub(Sub),
    SubGift(SubGift),
    Raid(Raid),
    Ritual(String),
}
