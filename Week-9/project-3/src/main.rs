use std::io::Write;
fn main() {
    let commisioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("Convicted Commisioners in Nigeria.txt").expect("Create failed");

    file.write_all("Convicted Commisioners in Nigeria".as_bytes()).expect("Write failed");

    for i in 0..commisioner.len(){
        file.write_all(commisioner[i].as_bytes()).expect("Write failed");
        file.write_all(ministry[i].as_bytes()).expect("Write failed");
        file.write_all(geo_zone[i].as_bytes()).expect("Write failed");
    }
}
