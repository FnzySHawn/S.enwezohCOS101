use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};

fn main()  -> io::Result<()> {

    let mut file = File::create("high-quality.txt").expect("failed to create");

    file.write_all("Lager\t\t|stout\t\t|Non-Alcholic\n".as_bytes()).expect("failed to write");
    file.write_all("33 Export\t|legend\t\t|maltina\n".as_bytes()).expect("failed to write");
    file.write_all("desperados\t|turbo king\t|amstel malta\n".as_bytes()).expect("failed to write");
    file.write_all("Goldberg\t|Williams\t|Malta Gold\n".as_bytes()).expect("failed to write");
    file.write_all("gulder\t\t|        \t|Fayrouz\n".as_bytes()).expect("failed to write");
    file.write_all("Heineken\t|        \t|       \n".as_bytes()).expect("failed to write");
    file.write_all("star\t\t|        \t|       \n".as_bytes()).expect("failed to write");

    println!("saved document");

    let mut input = String::new();
    println!("would you like to view (y/n)");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let choice = input.trim().to_lowercase();

    if choice == "y" {
        let file = File::open("high-quality.txt").expect("failed to open");
        let read = BufReader::new(file);

        for line in read.lines() {
            println!("{}", line?);

        }

    }else {
        println!("ok have a nice day");
    }

    Ok(())
}

     
    
    
    