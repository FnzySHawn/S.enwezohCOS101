fn main() {

    let b:(i32,bool,f64) = (16,true,5.0);
    print(b);
}
fn print(x:(i32,bool,f64)) {

    println!("inside print method");

    let (age,is_male,cgpa) = x;
    println!("age is {},ismale? {},cgpa is {}", age,is_male,cgpa);

}