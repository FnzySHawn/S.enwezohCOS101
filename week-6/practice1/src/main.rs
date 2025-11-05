fn main() {
    let name = "aissha lawal";
    let uni:&str = "pan-atlantic university";
    let addr:&str = "km 52 lekki-epe expressway , ibeju-lekki lagos";
    println!("name: {} ", name);
    println!("university: {} \nAdress :{}", uni,addr);

    let department:&'static str = "computer science";
    let school:&'static str = "school of science and teachnology ";
    println!("department: {}, \nschool: {}", department,school);

}
