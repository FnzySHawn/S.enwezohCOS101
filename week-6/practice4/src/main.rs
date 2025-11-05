fn main() {

    let fullname = "chidubem john uweh";
    let department = "computer science";
    let uni = "pan atlantic university";

    let mut school = "school of science".to_string();

    school.push_str(" and technology");

    println!("my name is: {}", fullname);

    println!("the length my fullname is: {}", fullname.len());
    println!("i am a student of {} department", department);
    println!("{}", school);
    println!("{}", uni);

}