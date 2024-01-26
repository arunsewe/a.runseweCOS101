use std::io;
use std::io::Read;

fn main(){

println!("Welcome to the Globacom Nigeria Database");

println!("What Position do you hold:
          Administrator
          Project Manager
          Employee
          Customer
          Vendor");

let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Failed to execute");

if input1.trim() == "Administrator"{
let mut file = std::fs::File::open("globacom_db.sql").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print! ("{}", contents);
}
else if input1.trim() == "Project Manager"{
let mut pmfile = std::fs::File::open("PROJECT_tb.sql").unwrap();
let mut pmcontents = String::new();
pmfile.read_to_string(&mut pmcontents).unwrap();
print! ("{}", pmcontents);
}
else if input1.trim() == "Employee"{
let mut emfile = std::fs::File::open("EMPLOYEES_tb.sql").unwrap();
let mut emcontents = String::new();
emfile.read_to_string(&mut emcontents).unwrap();
print! ("{}", emcontents);
}
else if input1.trim() == "Customer"{
let mut cufile = std::fs::File::open("CUSTOMER_tb.sql").unwrap();
let mut cucontents = String::new();
cufile.read_to_string(&mut cucontents).unwrap();
print! ("{}", cucontents);
}
else if input1.trim() == "Vendor"{
let mut vefile = std::fs::File::open("DATAPLAN_tb.sql").unwrap();
let mut vecontents = String::new();
vefile.read_to_string(&mut vecontents).unwrap();
print! ("{}", vecontents);
}
else{
    println!("No data for you");
}

println!("Thank you for using our Database.");

 }