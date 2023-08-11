fn main() {
    let mut p: u8 = 0;
    println!("{} p: {:p}", std::process::id(), &p);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        p = p + 1;
        println!("{} p: {} {:p}", std::process::id(), &p, &p); // a4
    }
}
