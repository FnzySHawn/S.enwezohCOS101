use std::io;
fn main() {
    println!("0- office admin\n1 - academic\n2 - lawyer\n3 - teacher");

    let mut inp1 = String::new();
    println!("please input the number matching ur public service");
    io::stdin().read_line(&mut inp1).expect("failed to read input ");
    let index:usize = inp1.trim().parse().expect("kindly enter valid input sir");

    let o_a = vec!["intern", "administrator", "senior administrator", "office manager", "director", "CEO"];
    let aca = vec!["-", "research assistant", "PhD Candidate", "post-doc researcher", "senior lecturer", "dean"];
    let law = vec!["paralegal", "junior associate", "associate", "senior associate 1 - 2", "senior ssociate 3-4", "partner"];
    let teach = vec!["placement", "classroom teacher", "snr teacher", "leading teacher", "deputy teacher", "principal"];

    let role = match index {
        0 => &o_a,
        1 => &aca,
        2 => &law,
        3 => &teach,
        _=> {
            println!("enter valid input");
            return
        }
    };

    
    if role == &o_a {
        for i in 0..=5{
            println!("{} {}", i,o_a[i]);

        }
    }else if role == &aca {
        for i in 0..=5{
            println!("{} {}", i,aca[i]);
        }
    }else if role == &law {
        for i in 0..=5{
            println!("{} {}", i,law[i]);
        }
    }else if role == &teach{
        for i in 0..=5{
            println!("{} {}", i,teach[i]);
        }
    }else{
        println!("pick a valid option sir/ma");
    }

    let mut inp2 = String::new();
    println!("kindly pick your position from 0 - 5");
    io::stdin().read_line(&mut inp2).expect("failed to read input");
    let index2:usize = inp2.trim().parse().expect("enter valid number"); 


    let job = role[index2];

    let mut inp3 = String::new();
    println!("how many years of experience do you have ");
    io::stdin().read_line(&mut inp3).expect("failed to read input");
    let exp:u16 = inp3.trim().parse().expect("enter a valid number");

    if index2 == 4 && exp >= 10 {
        println!("dear {}  your APS level is EL2 10-13 thank you for your service", job);
    }else if index2 == 3 && exp >= 8  {
        println!("dear {} APS level is EL1 8-10 thank you for ur service", job);
    }else if index2 == 2 && exp >= 5 {
        println!("dear {} your APS level is APS 5-8", job);
    }else if index2 == 1 && exp >= 3 {
        println!("dear {} your APS level is APS 3-5", job);
    }else if index2 == 0 && exp >= 1 {
        println!("dear {} your APS level is APS 1-2",job );
    }else if index2 == 5 {
        println!("Dear {} you are Senior Excecutive Service", job);
    }else {
        println!("you lack job expericene as an {} sorry ", job);
    }




}