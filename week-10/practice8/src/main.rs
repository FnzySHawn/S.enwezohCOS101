struct Employee {
    ceo:String,
    company:String,
    age:u32
} 
fn main() {

    let emp1 = Employee{
        company:String::from("microsoft corporation"),
        ceo:String::from("satya nadelaa"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("google inc."),
        ceo:String::from("sundai pichai"),
        age:51
    };

    display(emp1);
    display(emp2);

}

    fn display(emp:Employee) {
        println!("name is :{} company is {} age is {}", emp.ceo,emp.company,emp.age);

    }

