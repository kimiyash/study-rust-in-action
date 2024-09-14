#![allow(unused_variables)]

use std::mem::MaybeUninit;
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.message.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    message: Vec<Message>,
}

struct GrandStation;

impl GrandStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.message.push(msg);
    }
}
fn check_status(sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let base = GrandStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox {
            message: vec![],
        },
    };

    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!"));
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg);
}