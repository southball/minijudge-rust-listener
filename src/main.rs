mod cli;

use cli::Opts;
use clap::derive::Clap;

fn main() {
    let opts = Opts::parse();

    let context = zmq::Context::new();
    let requester = context.socket(zmq::SUB).unwrap();

    requester.connect(&opts.socket).expect("Failed to connect to socket.");
    requester.set_subscribe(b"").expect("Failed to set subscription.");

    let mut msg = zmq::Message::new();
    loop {
        requester.recv(&mut msg, 0).unwrap();
        println!("Received message: {}", msg.as_str().unwrap());
    }
}
