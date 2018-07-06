use super::Result;
use super::{ClearChat, GlobalUserState, PrivMsg, RoomState, UserNotice, UserState};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    CLEARCHAT(ClearChat),
    GLOBALUSERSTATE(GlobalUserState),
    PRIVMSG(PrivMsg),
    ROOMSTATE(RoomState),
    USERNOTICE(UserNotice),
    USERSTATE(UserState),
    HOSTTARGET(String),
    PING,
    Other(String),
}

impl Command {
    pub fn parse(command: &str, tags: Vec<String>) -> Result<Command> {
        let tags: HashMap<&str, Option<&str>> = tags
            .iter()
            .map(|tag| {
                let mut split = tag.split('=');
                let tag_name = split.next().unwrap();
                let tag_value = if let Some(value) = split.next() {
                    if value != "" {
                        Some(value)
                    } else {
                        None
                    }
                } else {
                    None
                };

                (tag_name, tag_value)
            })
            .collect();
            
        match command {
            "CLEARCHAT" => Ok(Command::CLEARCHAT(ClearChat::parse(tags)?)),
            "GLOBALUSERSTATE" => Ok(Command::GLOBALUSERSTATE(GlobalUserState::parse(tags)?)),
            "PRIVMSG" => Ok(Command::PRIVMSG(PrivMsg::parse(tags)?)),
            "ROOMSTATE" => Ok(Command::ROOMSTATE(RoomState::parse(tags)?)),
            // "USERNOTICE" => Ok(Command::USERNOTICE(UserNotice::parse(tags)?)),
            // "USERSTATE" => Ok(Command::USERSTATE(UserState::parse(tags)?)),
            "PING" => Ok(Command::PING),
            _ => Ok(Command::Other(command.to_owned())),
        }
    }
}
