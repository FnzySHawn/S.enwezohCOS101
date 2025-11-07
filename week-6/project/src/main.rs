use std::io;
fn main() {
    println!("-------------------------------------------");
    println!("\tMenu\t\t\t\tprice");
    println!("P = Poundo Yam/Edinkaiko soup\t\tN3,200");
    println!("F = Fried rice & chicken\t\tN3,000");
    println!("A = Amala & Ewedu soup\t\t\tN2,500");
    println!("E = Eba & Egusi Soup\t\t\tN2,000");
    println!("W = White Rice & Stew\t\t\tN2,500");
    println!("-------------------------------------------");

    let mut inp1 = String::new();
    let mut inp2 = String::new();

    println!("kindly enter a food code of choice from the menu");
    io::stdin().read_line(&mut inp1).expect("failed to read input");

    println!("how many do you wish to buy");
    io::stdin().read_line(&mut inp2).expect("failed to read input");

    let code = inp1.trim().to_uppercase();
    let qty:u32 = inp2.trim().parse().expect("enter a valid number");

    let price = match code.as_str() {
        "P" => 3_200,
        "F" => 3_000,
        "A" => 2_500,
        "E" => 2_000,
        "W" => 2_500,
        _=> {
            println!("enter a valid code sir");
                return
            }
        };

        let tot = price * qty; 

        if tot > 10_000 {
            let amt = tot * ( 5 / 100 );
            println!("your total amout to pay is {}", tot - amt);
        }else {
            println!("your total amount to pay is {}", tot);
        }
    }
