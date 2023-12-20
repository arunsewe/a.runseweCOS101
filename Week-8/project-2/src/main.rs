use std::io;
fn main() {
let mut candidate_name: Vec<String> = Vec::new();
let mut candidate_exp: Vec<i64> = Vec::new();

println!("Enter Total Number Of Candidates:");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Failed to read input");
let num: i64 = input1.trim().parse().expect("Invalid Input");

for _i in 0..num {
println!("What is your government name?");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("Failed to read input");
let name: String = input2.trim().parse().expect("Invalid Input");
candidate_name.push(name.clone());

println!("How many years of experience in programming do you have?");
let mut input3 = String::new();
io::stdin().read_line(&mut input3).expect("Failed to read input");
let exp: i64 = input3.trim().parse().expect("Invalid Input");
candidate_exp.push(exp); 

let max_value = candidate_exp.iter().max().unwrap_or(&0);

if exp == *max_value {
    println!("The candidate with the highest years of experience is: {}", name);
   
 }

}
}
