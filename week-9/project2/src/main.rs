use std::io::{self, Write};
use std::fs::OpenOptions;

fn get_user() -> (String, String, String, String) {
    let mut name = String::new();
    let mut matric = String::new();
    let mut dept = String::new();
    let mut level = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter matric number:");
    io::stdin().read_line(&mut matric).unwrap();

    println!("Enter department:");
    io::stdin().read_line(&mut dept).unwrap();

    println!("Enter level (100/200/300/400):");
    io::stdin().read_line(&mut level).unwrap();

    (name.trim().to_string(),
     matric.trim().to_string(),
     dept.trim().to_string(),
     level.trim().to_string())
}

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // start fresh
        .open("SMIS.txt")
        .expect("failed to create");

    writeln!(file, "\t\tPAU SMIS\n").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15} {:<10}", 
        "Student Name", "Matric Number", "Department", "Level").unwrap();
    writeln!(file, "-------------------------------------------------------------").unwrap();
    let mut students: Vec<(String, String, String, String)> = Vec::new();

    loop {
        println!("\nEnter Student Details:");
        let student = get_user();

        // push tuple into vector
        students.push(student);

        // Ask if another student should be added
        let mut again = String::new();
        println!("Add another student? (y/n)");
        io::stdin().read_line(&mut again).unwrap();

        if again.trim().to_lowercase() == "n" {
            break;
        }
    }
    for s in &students {
        writeln!(file, "{:<20} {:<15} {:<15} {:<10}", 
            s.0, s.1, s.2, s.3).unwrap();
    }
    println!("\nSaved! Here are all the students:\n");
    for s in students {
        println!("{:<20} {:<15} {:<15} {:<10}", 
            s.0, s.1, s.2, s.3);
    }
}