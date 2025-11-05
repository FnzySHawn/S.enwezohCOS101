fn main() {
    let name1 = "ayomide adesokan";
    println!("my name is {}", name1);

    let name2 = name1.replace("ayomide","adebare");
    println!("you can also call me  {}", name2);
    let faculty  = "faculty of science and technology";

    let school = faculty.replace("faculty", "school");
    println!("i am a student of the {}", school);

}