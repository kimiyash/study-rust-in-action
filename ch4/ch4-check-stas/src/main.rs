use std::rc::Rc;
use std::cell::RefCell;

#[derive(Copy, Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Clone for CubeSat {
    fn clone(&self) -> Self {{
        CubeSat { id: self.id }
    }}
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
    let sat_ids = fetch_sat_ids();

    println!("base: {:?}", base);
    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.34;
        println!("base2: {:?}", base2);
    }
    println!("base: {:?}", base);   

    for sat_id in sat_ids {
        let msg = Message { to: sat_id, content: String::from("hello") };
        base.borrow().send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.borrow().connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}