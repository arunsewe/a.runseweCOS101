use std::io;
fn main() {

    println!("Welcome! \nWhat calculation would you like to perform:");
    let mut ans1 = String::new();
    io::stdin().read_line(&mut ans1).expect("Failed to read input");
    let ans :String = ans1.trim().parse().expect("Failed to read input");


   if ans == "area of trapezium" {
   

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter Height:");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let h1: f32 = input1.trim().parse().expect("Failed to read input");

   println!("Enter Base A:");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let a: f32 = input2.trim().parse().expect("Failed to read input");

   println!("Enter Base B:");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let b: f32 = input3.trim().parse().expect("Failed to read input");

   let aot = h1 / 2.0 * (a + b);
   println!("Area of Trapezium = {}",aot);

} else if ans == "area of rhombus" {

   

   let mut input4 = String::new();
   let mut input5 = String::new();

   println!("Enter Diagonal A:");
   io::stdin().read_line(&mut input4).expect("Failed to read input");
   let d1: f32 = input4.trim().parse().expect("Failed to read input");

   println!("Enter Diagonal B:");
   io::stdin().read_line(&mut input5).expect("Failed to read input");
   let d2: f32 = input5.trim().parse().expect("Failed to read input");

   let aor = 0.5 * d1 * d2;
   println!("Area of Rhombus = {}", aor);

} else if ans == "area of parallelogram"{

   
   
   let mut input6 = String::new();
   let mut input7 = String::new();

   println!("Enter Base:");
   io::stdin().read_line(&mut input6).expect("Failed to read input");
   let b: f32 = input6.trim().parse().expect("Failed to read input");

   println!("Enter Altitude:");
   io::stdin().read_line(&mut input7).expect("Failed to read input");
   let a: f32 = input7.trim().parse().expect("Failed to read input");

   let aop:f32 = b * a;
   println!("Area of Parallelogram = {}",aop);

} else if ans == "area of cube" {


   let mut input8 = String::new();

   println!("Enter Length:");
   io::stdin().read_line(&mut input8).expect("Failed to read input");
   let z: f32 = input8.trim().parse().expect("Failed to read input");

   let aoc: f32 = 6.0 * z * z ;
   println!("Area of Cube = {}", aoc);

}else if ans == "volume of cylinder" {


   let mut input9 = String::new();
   let mut inp1 = String::new();

   println!("Enter Radius:");
   io::stdin().read_line(&mut input9).expect("Failed to read input");
   let r: f32 = input9.trim().parse().expect("Failed to read input");

   println!("Enter Height:");
   io::stdin().read_line(&mut inp1).expect("Failed to read input");
   let h: f32 = inp1.trim().parse().expect("Failed to read input");

   let voc:f32 = 3.14 * r * r * h;
   println!("Volume of Cylinder = {}",voc);
}

   



}
