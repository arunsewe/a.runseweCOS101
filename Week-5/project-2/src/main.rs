use std::io;
fn main() {
    let mut input1 = String::new();
    let mut num = String::new();

    let a = 1_560_000;
    let b = 1_480_000;
    let c = 1_300_000;
    let d = 100_000;

    println!("Are you experienced (True/False)");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let exp: bool = input1.trim().parse().expect("Not a valid input");

    println!("Enter you age");
    io::stdin().read_line(&mut num).expect("Not a valid input");
    let age :i32 = num.trim().parse().expect("Not a valid input");

    if exp == false { 
        println!("Your incentive is {}", d);
    }

     
     else if age >= 40 {
        println!("Your incentive is {}",a );
    } else if age >= 30 && age <40 {
        println!("Your incentive is {}", b);
    } else if age < 30 {
        println!("Your incentive is {}", c);
    } 
}