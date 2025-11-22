use std::io;
fn get_user() -> (String, u32) {
    let mut name = String::new();
    println!("Enter applicant name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let mut exp = String::new();
    println!("Enter years of programming experience:");
    io::stdin().read_line(&mut exp).expect("Failed to read input");

    let exp: u32 = exp.trim().parse().expect("Please enter a valid number");

    (name.trim().to_string(), exp)
}
fn highest_experience(users: &Vec<(String, u32)>) -> &(String, u32) {
    let mut highest = &users[0];

    for u in users {
        if u.1 > highest.1 {
            highest = u;
        }
    }
    highest
}

fn main() {
    let mut users: Vec<(String, u32)> = Vec::new();

    println!("How many applicants are being interviewed?");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read input");

    let count: usize = count.trim().parse().expect("Enter a valid number");

    for _ in 0..count {
        println!("\n--- Enter applicant details ---");
        let user = get_user();
        users.push(user);
    }

    let top = highest_experience(&users);

    println!( "\n The applicant with the highest programming experience is: {} ({} years)",
        top.0, top.1
    );
}