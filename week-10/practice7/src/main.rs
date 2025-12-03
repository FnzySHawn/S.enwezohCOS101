struct Employee {
    name:String,
    company:String,
    age:u32

}

fn main() {
    let emp1 = Employee {
        company:String::from("ernst & young"),
        name:String::from("ebibiong jessica"),
        age:25

    };
    println!("name = {} \n", emp1.name);
    println!("company = {} \n", emp1.company);
    println!("age = {}", emp1.age);

}
