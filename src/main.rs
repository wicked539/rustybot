extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let cfg = Config {
        nickname: Some(format!("rustybot")),
        server: Some(format!("irc.synyx.de")),
        channels: Some(vec![format!("#irchacks")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        // Do message processing.
    }
    let foo = server.send_privmsg("#irchacks", "yolo").unwrap();

    println!(stringify!(&foo));
}