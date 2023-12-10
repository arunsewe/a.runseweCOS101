use std::io;
fn main (){
	let mut inputs:Vec<String> = Vec::new();

	println!("How many siblings do you have?");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let no_of_siblings:i32 = input1.trim().parse().expect("Invalid Input");



	for i in 0..=no_of_siblings {
		println!("First name of siblings:");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let name = input2.trim().to_string();

		inputs.push(name);

		println!("Age of siblings:");
		let mut input3 = String::new();
		io::stdin().read_line(&mut input3).expect("Failed to read input");
		let age: i32= input2.trim().parse().expect("Invalid Input");

		let age1:String = age.to_string();
		inputs.push(age1);

	

	if age > 18 {
		println!("Are you married? (Married or Single)");
		let mut input4 = String::new();
		io::stdin().read_line(&mut input4).expect("Failed to read input");
		let marital_status = input4.trim().to_string();
		inputs.push(marital_status.clone());

		if marital_status == "Single" {
			println!("Are you a student or worker?");
			let mut input5 = String::new();
			io::stdin().read_line(&mut input5).expect("Failed to read input");
			let occ_status = input5.trim().to_string();
			inputs.push(occ_status.clone());

			if occ_status == "student" {
				println!("What university do you attend?");
				let mut input6 = String::new();
				io::stdin().read_line(&mut input6).expect("Failed to read input");
				let uni = input6.trim().to_string();
				inputs.push(uni);

				println!("Course of Study:");
				let mut input7 = String::new();
				io::stdin().read_line(&mut input7).expect("Failed to read input");
				let course = input7.trim().to_string();
				inputs.push(course);

			}
		} else if marital_status == "Married" {
			println!("Do you have any offsprings?");
			let mut input8 = String::new();
			io::stdin().read_line(&mut input8).expect("Failed to read input");
			let off_spr = input8.trim().to_string();
			inputs.push(off_spr);

			println!("What city does your family live in?");
			let mut input9 = String::new();
			io::stdin().read_line(&mut input9).expect("Failed to read input");
			let city = input9.trim().to_string();
			inputs.push(city);

		}


	} else if age < 18 {
		println!("Have you written WAEC? (Yes or No)");
		let mut inp1 = String::new();
		io::stdin().read_line(&mut inp1).expect("Failed to read input");
		let waec =inp1.trim().to_string();
		inputs.push(waec.clone());

		if waec =="Yes" {
			println!("What secondary school did you attend?");
			let mut inp2 = String::new();
			io::stdin().read_line(&mut inp2).expect("Failed to read input");
			let school = inp2.trim().to_string();
			inputs.push(school);

		} else if waec =="No" {
			println!("What is your current class level?");
			let mut inp3 = String::new();
			io::stdin().read_line(&mut inp3).expect("Failed to read input");
			let class = inp3.trim().to_string();
			inputs.push(class);
		}
	}
	println!("Stored Information:");
	for (index, value) in inputs.iter().enumerate() {
        println!("Sibling {}: {}", index + 1, value);
    }

	}
}
