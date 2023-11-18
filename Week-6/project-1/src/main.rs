use std::io;
fn main() {


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    

    
   let mut count:i32 = 0;
   loop {
    count += 1;
   


    println!("Name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name: String = input1.trim().parse().expect("Failed to read input");


    println!("Email:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let email: String = input2.trim().parse().expect("Failed to read input");

    println!("Department:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let department: String = input3.trim().parse().expect("Failed to read input");

    println!("Level:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let level: i32 = input4.trim().parse().expect("Failed to read input");


    println!("State Of Origin:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let soo: String = input5.trim().parse().expect("Failed to read input");

    println!("CGPA:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let cgpa: f32 = input6.trim().parse().expect("Failed to read input");

    println!("Are you a class representative?");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let rep: String = input7.trim().parse().expect("Failed to read input");


    if rep == "Yes" && level > 100 && cgpa > 4.0 {
        println!("{} \n{} \n{} \n{} \nYou can vote!", name, email, department, soo);
    

    } else {
        println!("{} \n{} \n{} \n{} \nSorry, you are not eligible", name, email, department, soo);
    }

    if count == 150 {
        break
    }
}





    

}
