use std::io;

fn main() {
    println!("Choose a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Volume of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_number("Enter choice: ");

    match choice {
        1.0 => calc_trapezium(),
        2.0 => calc_rhombus(),
        3.0 => calc_parallelogram(),
        4.0 => calc_cube(),
        5.0 => calc_cylinder(),
        _ => println!("Invalid choice."),
    }
}

fn read_number(prompt: &str) -> f64 {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn calc_trapezium() {
    let height = read_number("Enter height: ");
    let base1  = read_number("Enter base1: ");
    let base2  = read_number("Enter base2: ");

    let area = height * (base1 + base2) / 2.0;
    println!("Area of trapezium = {}", area);
}

fn calc_rhombus() {
    let d1 = read_number("Enter diagonal 1: ");
    let d2 = read_number("Enter diagonal 2: ");

    let area = 0.5 * d1 * d2;
    println!("Area of rhombus = {}", area);
}

fn calc_parallelogram() {
    let base = read_number("Enter base: ");
    let altitude = read_number("Enter altitude: ");

    let area = base * altitude;
    println!("Area of parallelogram = {}", area);
}

fn calc_cube() {
    let side = read_number("Enter side length: ");

    let volume = 6.0 * side.powf(3.0);
    println!("Volume of cube = {}", volume);
}

fn calc_cylinder() {
    let radius = read_number("Enter radius: ");
    let height = read_number("Enter height: ");

    let volume = std::f64::consts::PI * radius.powf(2.0) * height;
    println!("Volume of cylinder = {}", volume);
}
