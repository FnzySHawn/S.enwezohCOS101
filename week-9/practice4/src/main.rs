use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use std::io;

fn main() {
    let mut file = fs::File::create("data.txt").expect("failed to create");
    file.write_all("welcome to practice4".as_bytes()).expect("failed to write");
    let mut app = OpenOptions::new().append(true).open("data.txt").expect("cannot open");
    let mut inp1 = String::new();
    println!("what would u like to append");
    io::stdin().read_line(&mut inp1).expect("failed to read input");
    let  u_app = inp1.trim();
    app.write_all("\nHello class".as_bytes()).expect("write failed");
    app.write_all("\nThis is the appendage to the document\n".as_bytes()).expect("write failed");
    app.write_all(u_app.as_bytes()).expect("write failed");
    println!("file append success");

}



