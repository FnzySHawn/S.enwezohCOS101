use std::io;
use std::io::Read;

fn get_role(prompt:&str) -> String {
    let mut inp1 = String::new();
    println!("{prompt}");
    io::stdin().read_line(&mut inp1).expect("failed to read");
    inp1.trim().to_string()

}

    fn admin() {
        let mut file = std::fs::File::open("globacom_dbase.sql").expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to open");
        println!("{}", contents);
    }
    fn vendor() {
        let mut file = std::fs::File::open("dataplan_tb.sql").expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to open");
        println!("{}", contents);
    }
    
    fn p_m() {
        let mut file = std::fs::File::open("project_tb.sql").expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to open");
        println!("{}", contents);
    }
    fn emp() {
        let mut file = std::fs::File::open("staff_tb.sql").expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to open");
        println!("{}", contents);
    }
    fn  cust() {
        let mut file = std::fs::File::open("customer_tb.sql").expect("failed to open");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to open");
        println!("{}", contents);
    }

fn main() {
    println!("hello there");

    let mut inp2 = String::new();
    println!("what is your name ");
    io::stdin().read_line(&mut inp2).expect("failed to read input");
    let name = inp2.trim();

    let mut inp3 = String::new();
    println!("kindly enter the secret password: ");
    io::stdin().read_line(&mut inp3).expect("failed to read input");
    let password = inp3.trim();

    if password == "cos101" {
        println!("welcome {} you look beautiful today", name);
        let role = get_role("what is your role in the organisation");


     match role.as_str() {
        "administrator" => admin(),
        "project manager" => p_m(),
        "employee" => emp(),
        "customer" => cust(),
        "vendor" => vendor(),
         _ => {
            println!("please ensure your answer is either administrator\nproject manager\nemployee\ncustomer\nvendor");
        }
    }


    }else {
        println!("wrong password try again ");
   }

}