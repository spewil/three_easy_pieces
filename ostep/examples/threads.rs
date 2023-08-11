use std::env;
use std::thread;

static mut COUNTER: u16 = 0;

fn worker(loops: u16) {
    let mut i: u16 = 0;
    while i < loops {
        unsafe {
            COUNTER += 1;
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run --example threads <string>")
    }
    let loops: u16 = args[1].to_string().parse().unwrap();
    let handler1 = thread::spawn(move || worker(loops));
    let handler2 = thread::spawn(move || worker(loops));

    let _ = handler1.join();
    let _ = handler2.join();

    unsafe {
        println!("{}", COUNTER);
    }
}
