use std::io;
fn main() {
    let mut input_1 = String::new();
    println!("how old are you");
    io::stdin().read_line(&mut input_1).expect("failed to read input");
    let age:u32 = input_1.trim().parse().expect("please enter a valid age bruh");
    let mut exp = String::new();
    println!("are you experienced? (y/n): ");
    io::stdin().read_line(&mut exp).expect("pick y or n");

    let experience = exp.trim().to_lowercase();

    if experience == "y" && age >= 40 {
        println!("your incentive is N1_560_000");
    }else if experience == "y" && age >= 30 {
        println!("your incentive is N1_480_000");
    }else if experience == "y" && age < 28 {
        println!("your incentive is N1_300_000");
    }else {
    println!("your incentive is N100_000");
    }
} 