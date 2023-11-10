use std::io;
fn main() {
    let mut input1 = String::new();

    println!("Enter your height(in cm)");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are average height");

    } else if height > 170.0 && height <=195.0
    {
        println!("You are tall");

    } else if height < 150.0 && height > 100.0
    {
        println!("You are dwarf");
    } else 
    {
        println!("You are abnormal height");
    }
}
