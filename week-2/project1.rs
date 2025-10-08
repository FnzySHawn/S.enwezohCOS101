fn main () {
	let pri:f64 = 520_000_000.0;
	let ra:f64 = 10.0;
	let n:f64 = 5.0;

	// lets assign the a variable to the formula for comp intest
let a = pri * (1.0 + ra / 100.0).powf(n);
println!("amount is {}", a);
let comp_int = a - pri;
println!("compund intrest is {}", comp_int);

}