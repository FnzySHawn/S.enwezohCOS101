use std::io
fn main() {
	let mut base_input = string::new();
	let mut exponent_input = string::new();
	println!("what is the base ");
	io::stdin.read_line(&mut base_input).expect("your input is invalid");
	io::stdin.read_line(&mut exponent_input).expect("your input is invalid");

	let base:f64 = base_input.trim().parse().expect("enter valid input bruh");
	let exponent:f64 = exponent_input.trim().parse().expect("enter valid input");

	let power = base.powf(exponent);
	println!("{} to the power of {} is {}", base,exponent,power);
}

