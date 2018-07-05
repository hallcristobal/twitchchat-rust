use super::Command;
use std::io::{Error, ErrorKind};

#[derive(Eq, PartialEq, Debug)]
pub struct Message {
    user_name: String,
    message: String,
    channel: String,
    command: Option<Command>,
}

impl Message {
    pub fn parse(msg: &str) -> ::std::io::Result<Message> {
        let mut position = 0;

        let tags = if msg.starts_with("@") {
            if let Some(next_space) = msg.find(" ") {
                position = next_space;
                Some(
                    msg[1..next_space]
                        .split(";")
                        .map(|tag| tag.to_owned())
                        .collect::<Vec<String>>(),
                )
            } else {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        } else {
            None
        };

        let message = msg[position..].trim();
        let prefix = if message.starts_with(":") {
            if let Some(next_space) = message.find(" ") {
                position = next_space + 1;
                Some(&message[..next_space])
            } else {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        } else {
            None
        };

        let user_name = if let Some(prefix) = prefix {
            if let Some(exclaim) = prefix.find("!") {
                String::from(&prefix[1..exclaim])
            } else {
                String::from(&prefix[1..])
            }
        } else {
            String::new()
        };

        let message = message[position..].trim();
        let command = if let Some(next_space) = message.find(" ") {
            position = next_space + 1;
            Some(&message[..next_space])
        } else {
            if msg.len() > position {
                Some(message)
            } else {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        };

        let message = message[position..].trim();
        let channel = if let Some(next_space) = message.find(" ") {
            if message.starts_with("#") {
                position = next_space + 1;
                String::from(&message[..next_space])
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let mut raw_message = None;
        if position < message.len() {
            let mut message = message[position..].trim();
            loop {
                if message.starts_with(':') {
                    raw_message = Some(&message[1..]);
                    break;
                }

                if let Some(next_space) = message.find(" ") {
                    position = next_space;
                    message = &message[position..].trim();
                    continue;
                } else {
                    break;
                }
            }
        }
        Ok(Message {
            user_name,
            message: raw_message.unwrap_or("").to_owned(),
            channel,
            command: if let (Some(tags), Some(command)) = (tags, command) {
                match Command::parse(command, tags) {
                    Ok(c) => {
                        println!("{:?}", c);
                        Some(c)
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                }
            } else {
                None
            },
        })
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn command(&self) -> &Option<Command> {
        &self.command
    }
}
