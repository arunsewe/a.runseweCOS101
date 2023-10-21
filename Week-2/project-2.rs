fn main() {
	let t:f64= 450_000.00 * 2.0;
	let m:f64 = 1_500_000.00 * 1.0;
	let h:f64 = 750_000.00 * 3.0;
	let d:f64 = 2_850_000.00 * 3.0;
	let a:f64 = 250_000.00 * 1.0;

	let s = t + m + h + d + a ;
	let a = s / 10.0;

	println!("The sum is {}.", s);
	println!("The average is {}.", a);
}