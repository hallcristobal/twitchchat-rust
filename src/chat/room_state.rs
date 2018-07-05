#[derive(Eq, PartialEq, Debug)]
pub struct RoomState {
    braodcaster_language: Option<String>,
    r9k: Option<bool>,
    slow: Option<bool>,
    subs_only: Option<bool>,
}
