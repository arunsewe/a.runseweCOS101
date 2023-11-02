use std::io;

fn main() {
    println!("Speedometer");

   let mut input1 = String::new();
   let mut input2 = String::new();

   println!("Enter Mileage");
   io::stdin().read_line(&mut input1).expect("Invalid input");
   let a:f32 = input1.trim().parse().expect("Invalid input");

   println!("Enter Duration of Mileage");
   io::stdin().read_line(&mut input2).expect("Invalid input");
   let b:f32 = input2.trim().parse().expect("Invalid input");

   let s:f32 = (a * 1.61) / b ;
    println!("Speed of Car: {}km/hr",s);

}
