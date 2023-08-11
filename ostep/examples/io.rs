use std::fs;

fn main() {
    let filename = "/tmp/file.txt";
    match fs::write(filename, "Hello world") {
        Err(why) => panic!("couldn't write to {}: {}", filename, why),
        Ok(_) => println!("successfully wrote to {}", filename),
    }
}
