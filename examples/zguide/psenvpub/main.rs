#![crate_name = "psenvpub"]

use std::thread;
use std::time::Duration;

fn main() {
    //prepare context and publisher
    let context = zmq2::Context::new();
    let publisher = context.socket(zmq2::PUB).unwrap();
    publisher
        .bind("tcp://*:5563")
        .expect("failed binding publisher");

    loop {
        publisher
            .send("A", zmq2::SNDMORE)
            .expect("failed sending first envelope");
        publisher
            .send("We don't want to see this", 0)
            .expect("failed sending first message");
        publisher
            .send("B", zmq2::SNDMORE)
            .expect("failed sending second envelope");
        publisher
            .send("We would like to see this", 0)
            .expect("failed sending second message");
        thread::sleep(Duration::from_millis(1));
    }
}
