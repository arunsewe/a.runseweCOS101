use std::io;
fn main() {
    println!("Menu
        \np = Poundo Yam/Edinkaiko Soup  -  N3,200
        \nf = Fried Rice and Chicken     -  N3,000
        \na = Amala and Ewedu Soup       -  N2,500
        \ne = Eba and Egusi Soup         -  N2,000
        \nw = White Rice and Stew        -  N2,500");

    let p = 3_200;
    let f = 3_000;
    let a = 2_500;
    let e = 2_000;
    let w = 2_500;

    println!("Do you want Pounded yam/Edinkaiko Soup? (True/False)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let ans1: bool= input1.trim().parse().expect("Failed to read input");

    if ans1 == true {
        println!("Enter quantity");
    }
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let  q1: i32 = input2.trim().parse().expect("Failed to read input"); 
    let u = p * q1;
 

    println!("Do you want Fried Rice and Chicken? (True/False)");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let ans2: bool = input3.trim().parse().expect("Failed to read input");

    if ans2 == true {
        println!("Enter quantity");
    }
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let  fqty: i32 = input4.trim().parse().expect("Failed to read input");
    let x = f * fqty;


    println!("Do you want Amala and Ewedu Soup? (True/False)");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let ans3: bool= input5.trim().parse().expect("Failed to read input");

    if ans3 == true {
        println!("Enter quantity");
    }let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let  aqty: i32 = input6.trim().parse().expect("Failed to read input");
    let y = a * aqty;



    println!("Do you want Eba and Egusi Soup? (True/False)");

    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let anse: bool= input7.trim().parse().expect("Failed to read input");

    if anse == true {
        println!("Enter quantity");
    }
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let eqty: i32 = input8.trim().parse().expect("Failed to read input"); 
    let z = e * eqty;

    println!("Do you want White Rice and Stew? (True/False)");

    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let ans5: bool= input9.trim().parse().expect("Failed to read input");

    if ans5 == true {
        println!("Enter quantity");
    }
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let wqty: i32 = input10.trim().parse().expect("Failed to read input");
    let k = w * wqty;


    
    
        let t6 = u + y + x  + z + k ;

    if t6 > 10_000 {
        let t7 = t6 * (5/100);
        let t8 = t6 - t7;
        println!("Your total is {}", t8);
    }else if t6 < 10_000{
        println!("Your total is {}", t6);
    }
}
