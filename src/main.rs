#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate quick_error;

mod chat;
mod chat_connection;
use chat_connection::ChatConnection;

use chat::{Command, Message};

fn main() {
    let mut con = ChatConnection::connect(None).expect("Failed to connect");
    con.request_membership()
        .expect("Failed to request membership");
    con.request_tags().expect("Failed to request ags");
    con.request_commands().expect("Failed to request commands");
    con.login("justinfan123456", None).expect("Failed to login");
    con.join_channel("c_midknight")
        .expect("Failed to connect to channel");

    loop {
        let msg = con.raw_message().expect("Failed to recieve message");
        println!("{}", msg);
        // println!("{}", msg);
        let msg = Message::parse(&msg);
        println!("{:?}", msg);
        // let msg = Message::parse(&msg).expect("Couldn't parse message");
        // if *msg.command() == Command::PING {
        //     con.send_pong().expect("Failed to send pong");
        // }
        // println!("{}: {}", msg.user_name(), msg.message());
    }
}
