use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

#[derive(Copy, Debug)]
enum StatusMessage {
    Ok,
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
struct  Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipent: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipent.id {
                let msg = self.messages.remove(i);
                return Some(msg)
            }
        }
        None
    }
}

#[derive(Debug)]
struct GrandStation {
    radio_freq: f64,
}

impl GrandStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };
    let base = Rc::new(RefCell::new(
        GrandStation { radio_freq: 87.65 }
    ));

    println!("base: {:?}", base);
    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.34;
        println!("base2: {:?}", base2);
    }
    println!("base: {:?}", base);   

    let sat_ids = fetch_sat_ids();
    for sat_id in &sat_ids {
        let msg = Message { to: *sat_id, content: String::from("hello") };
        base.borrow().send(&mut mail, msg);
    }
    for sat_id in &sat_ids {
        let sat = base.borrow().connect(*sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}