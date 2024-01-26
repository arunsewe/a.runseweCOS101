use std::io::Write;
fn main() {
    
    let stout = vec!["Legend","  Turbo King","  Williams"];
    let lager = vec!["Desperados"," 33Export"," Goldberg"," Gulder"," Heineken"," Star"];
    let non_alcoholic = vec!["Maltina","  Amstel Malta","  Malta Gold","  Fayrouz"];

    let mut drinks = std::fs::File::create("Nigerian Breweries.txt").expect("create failed");
    
    for i in 0..stout.len() {
        drinks.write_all(stout[i].as_bytes()).expect("write failed");
    
    } drinks.write_all("\n".as_bytes()).expect("Write failed");
    
    for j in 0..lager.len() {
        drinks.write_all(lager[j].as_bytes()).expect("write failed");
    
    } drinks.write_all("\n".as_bytes()).expect("Write failed");

    for k in 0..non_alcoholic.len() {
        
        drinks.write_all(non_alcoholic[k].as_bytes()).expect("write failed");
    }

    println!("Data written to file");
}