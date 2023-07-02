#![allow(unused_variables)]       // <1>

#[derive(Debug)]
enum SatCubStatus {
    Ok,
}

#[derive(Debug)]                  // <2>
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    connect: String,
}

#[derive(Debug)]
struct GroundStation;

impl CubeSat {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
        }
    }
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("sat a: {:?}, status: {:?}", sat_id, SatCubStatus::Ok);
    sat_id
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mailbox = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat_id, connect: String::from("hello") };
        base.send(&mut mailbox, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);

        println!("{:?}: {:?}", sat, msg);
    }
}

