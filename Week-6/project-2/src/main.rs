use std::io;
fn main() {

    let mut count:i32 = 0;
    loop{
        count += 1;


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name = input1;

    println!("Number of research papers published:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let no:i32 = input2.trim().parse().expect("Failed to read input");

    if no >=3  && no <=5 {
        println!("{} \nYour incentive is N500,000",name);

    } else if no > 5 && no < 10 {
        println!("{} \nYour incentive is N800,000", name);
    } else if no > 10 {
        println!("{} \nYour incentive is N1,000,000", name);
    } else if no < 3 {
        println!("{} \nYour incentive is N100,000", name);
    }

    if count == 500 {
        break
    }
}




}
