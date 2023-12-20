fn main() {
    println!("PUBLIC SERVICE APS LEVEL CHECKER!!");

   let mut aps: Vec<String> = Vec::new();

   println!("How many people do you want to check?");
   let mut input4 = String::new();
   std::io::stdin().read_line(&mut input4).expect("Failed to read input");
   let num: i32 = input4.trim().parse().expect("Invalid Input");

    for i in 0..num {
    println!("Enter Your Government Name:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name: String = input1.trim().parse().expect("Invalid Input");
    aps.push(name.clone());


    println!("What department are you in?");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let department: String = input2.trim().parse().expect("Invalid Input");
    aps.push(department);
    

    println!("What position do you occupy?");
    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read input");
    let position: String = input3.trim().parse().expect("Invalid Input");
    aps.push(position.clone());

    if position == "Intern" {
        println!("{} holds position APS 1-2", name);
    } else if position == "Administrator"{
        println!("{} holds position APS 3-5", name);
    } else if position == "Senior Administrator"{
        println!("{} holds position APS 5-8", name);
    } else if position == "Office Manager" {
        println!("{} holds position EL1 8-10", name);
    } else if position == "Director" {
        println!("{} holds position EL2 10-13", name);
    } else if position == "CEO"{
        println!("{} holds position SES", name);
    } else if position == "Research Assistant"  {
        println!("{} holds position APS 3-5",name);
    } else if position == "PhD Candidate"{
        println!("{} holds position APS 5-8", name);
    } else if position == "Post-Doc Researcher" {
        println!("{} holds position EL1 8-10" ,name);
    } else if position == "Senior Lecturer" {
        println!("{} holds position EL2 10-13" ,name);
    } else if position == "Dean" {
        println!("{} holds position SES" ,name);
    } else if position == "Paralegal" {
        println!("{} holds position APS 1-2" ,name);
    } else if position == "Junior Associate" {
        println!("{} holds position APS 3-5" ,name);
    } else if position == "Associate" {
        println!("{} holds position APS 5-8" ,name);
    } else if position == "Senior Associate 1-2" {
        println!("{} holds position EL1 8-10" ,name);
    } else if position == "Senior Associate 3-4" {
        println!("{} holds position EL2 10-13" ,name);
    } else if position == "Partner" {
        println!("{} holds position SES" ,name);
    } else if position == "Placement" {
        println!("{} holds position APS 1-2" ,name);
    } else if position == "Classroom Teacher"{
        println!("{} holds position APS 3-5" ,name);
    } else if position == "Senior Teacher" {
        println!("{} holds position APS 5-8" ,name);
    } else if position == "Leading Teacher" {
        println!("{} holds position EL1 8-10" ,name);
    } else if position == "Deputy Principal" {
        println!("{} holds position EL2 10-13" ,name);
    } else if position == "Principal"{
        println!("{} holds position SES" ,name);
    }

}


    

}
