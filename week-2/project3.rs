fn main () {
	let pri:f64 = 510_000.0;
	let ra:f64 = 5.0;
	let n:f64 = 3.0;

	// lets assign the a variable to the formula for amount
let a = pri * (1.0 - ra / 100.0).powf(n);

println!("the value of the tv after 3 years  is {}", a);

}