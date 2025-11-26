use std::io::Write;
use std::fs;

fn main() {
    let mut file = fs::File::create("data.txt").expect("could not create");
    file.write_all("ez bot get better\n".as_bytes()).expect("failed to write");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");

}