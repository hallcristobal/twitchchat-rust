use chat::Result;
use std::fmt::Display;
use std::io::{BufRead, BufReader, Write};
use std::io::{Error, ErrorKind};
use std::net;

const TWITCH_SERVER: &'static str = "irc.chat.twitch.tv:6667";

pub struct ChatConnection {
    stream: net::TcpStream,
    nickname: String,
    reader: BufReader<net::TcpStream>,
    channel: Option<String>,
}

impl ChatConnection {
    pub fn connect(stream: Option<net::TcpStream>) -> Result<ChatConnection> {
        let stream = if let Some(stream) = stream {
            stream
        } else {
            net::TcpStream::connect(TWITCH_SERVER)?
        };

        let reader = BufReader::new(stream.try_clone()?);

        stream.set_read_timeout(None)?;

        Ok(ChatConnection {
            stream,
            nickname: String::new(),
            reader,
            channel: None,
        })
    }

    pub fn login(&mut self, nick: &str, pass: Option<&str>) -> Result<()> {
        if let Some(pass) = pass {
            self.send_raw(format!("PASS {}", pass))?;
        }
        self.send_raw(format!("NICK {}", nick))?;
        self.nickname = nick.to_owned();
        Ok(())
    }

    pub fn join_channel(&mut self, channel: &str) -> Result<()> {
        if self.channel.is_some() {
            return Err(Error::from(ErrorKind::AlreadyExists).into());
        }
        self.send_raw(format!("JOIN #{}", channel))?;
        self.channel = Some(channel.to_owned());
        Ok(())
    }

    pub fn part_channel(&mut self) -> Result<()> {
        if let Some(channel) = self.channel.take() {
            self.send_raw(format!("PART #{}", channel))?;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::NotConnected).into())
        }
    }

    pub fn request_membership(&mut self) -> Result<()> {
        self.send_raw("CAP REQ :twitch.tv/membership")?;
        Ok(())
    }

    pub fn request_tags(&mut self) -> Result<()> {
        self.send_raw("CAP REQ :twitch.tv/tags")?;
        Ok(())
    }

    pub fn request_commands(&mut self) -> Result<()> {
        self.send_raw("CAP REQ :twitch.tv/commands")?;
        Ok(())
    }

    pub fn raw_message(&mut self) -> Result<String> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(0) => Err(Error::from(ErrorKind::NotConnected).into()),
            Err(e) => Err(e.into()),
            _ => Ok(buf),
        }
    }

    pub fn send_raw<D: Display>(&mut self, msg: D) -> Result<()> {
        self.stream.write(format!("{}\r\n", msg).as_bytes())?;
        Ok(())
    }

    pub fn send_irc_memssage<D: Display>(&mut self, channel: D, msg: D) -> Result<()> {
        self.send_raw(format!("PRIVMSG #{} : {}", channel, msg))?;
        Ok(())
    }

    pub fn send_pong(&mut self) -> Result<()> {
        self.send_raw("PONG :tmi.twitch.tv")?;
        Ok(())
    }
}
