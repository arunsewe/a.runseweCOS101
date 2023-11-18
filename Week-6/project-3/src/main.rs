use std::io;

fn main() {
    let mut input1 = String::new();
    

    println!("Enter a number greater than 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input ");
    let n:u32 = input1.trim().parse().expect("Failed to read input");

    println!("Multiplication Table of {}",n);
    

    for i in 1..12 {
        for j in 1..n {
            let h = i * j;
            println!("{} x {} = {}", i, j, h );
            
        }
    }

    
}
