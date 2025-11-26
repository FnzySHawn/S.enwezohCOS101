use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::create("welcome_message.txt").expect("failed to create");
    file.write_all("welcome to practice 2\n".as_bytes()).expect("write failed");
    let mut open = std::fs::File::open("welcome_message.txt").expect("failed to open");
    let mut contents = String::new();
    open.read_to_string(&mut contents).expect("failed to read");
    print!("{}", contents);

}