use std::io::Write;
fn main() {
    
    let stud_name = vec!["Oluchi Mordi\n", "Adams Aliyu\n", "Shania Bolade\n", "Adekunle Gold\n", "Blanca Edemoh\n"];
    let matric_no = vec!["ACC10211111\n", "ECO10110101\n", "CSC10328828\n", "EEE11020202\n", "MEE10202001\n"];
    let department = vec!["Accounting\n", "Economics\n", "Computer\n", "Electrical\n", "Mechanical\n"];
    let level = vec!["300\n", "100\n", "200\n", "200\n", "100\n"];


    let mut file = std::fs::File::create("PAU Student Management Information System.txt").expect("create failed");
    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    
    for i in 0..stud_name.len(){
        file.write_all(stud_name[i].as_bytes()).expect("write failed");
        file.write_all(matric_no[i].as_bytes()).expect("write failed");
        file.write_all(department[i].as_bytes()).expect("write failed");
        file.write_all(level[i].as_bytes()).expect("write failed");

        println!("{}   {}   {}   {}", stud_name[i], matric_no[i], department[i], level[i]);
    }
    println!("\nFile created");
}
