// run with cargo run --example cpu

use std::env;
use std::{thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run --example cpu <string>")
    }
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("{:?}", args[1]);
    }
}
