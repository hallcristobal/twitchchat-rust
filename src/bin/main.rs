extern crate twitch_chat_lib;

use twitch_chat_lib::chat_connection::ChatConnection;

use twitch_chat_lib::chat::{Command, Message};

fn main() {
    let mut con = ChatConnection::connect(None).expect("Failed to connect");
    con.request_membership()
        .expect("Failed to request membership");
    con.request_tags().expect("Failed to request ags");
    con.request_commands().expect("Failed to request commands");
    con.login("justinfan123456", None).expect("Failed to login");
    con.join_channel("capitainetoinon")
        .expect("Failed to connect to channel");

    loop {
        let msg = con.raw_message().expect("Failed to recieve message");
        println!("{}", msg);
        let msg = Message::parse(&msg);
        println!("{:?}\n", msg);
    }
}
