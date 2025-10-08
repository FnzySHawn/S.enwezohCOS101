fn main() {
	let to:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let de:f64 =2_850_000.00;
	let ac:f64 = 250_000.00;

	let sum = to + mac + hp + de + ac;
	println!("sum of amounts is {} ", sum); 
   let avg = sum/5.0;
   println!("average is {}", avg);

}