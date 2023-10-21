fn main () {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	let a = p * (1.0 - (r / 100.0)).powf(n);
	println!("The depreciation is {}.", a);
	let v = p - a;
	println!("The value of the TV after 3 years is {}.", v);
}