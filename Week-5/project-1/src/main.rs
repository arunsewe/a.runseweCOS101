use std::io;
fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter value for b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter value for c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Failed to read input");

    let dis: f32 = (b * b) - a * c * 4.0;
    print!("The roots of the quadratic equation is: {}", dis);

    if dis > 0.0 {
         
        println!("\nThere are two distinct roots");
    }
    else if dis == 0.0 {
        println!("\nThere is exactly one real root");
    }
    else if dis < 0.0 {
        println!("\nThere are no real roots");
    }
}
