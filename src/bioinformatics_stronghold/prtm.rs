use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn prtm(filename :&str) -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string(filename)?;
	let mut hashmap = HashMap::new();
	let table = "A   71.03711
	C   103.00919
	D   115.02694
	E   129.04259
	F   147.06841
	G   57.02146
	H   137.05891
	I   113.08406
	K   128.09496
	L   113.08406
	M   131.04049
	N   114.04293
	P   97.05276
	Q   128.05858
	R   156.10111
	S   87.03203
	T   101.04768
	V   99.06841
	W   186.07931
	Y   163.06333";
	
	for line in table.lines(){
		let line_vec:Vec<&str> = line.split_whitespace().collect();
		hashmap.insert(line_vec[0].to_string(), line_vec[1].parse::<f64>().unwrap());
	}

	let mut total_mass = 0.0;
	for char in input.trim().chars(){
		total_mass += hashmap.get(&char.to_string()).unwrap();
	}

	println!("{}", total_mass);
	// for char in input.chars{

	// }
	Ok(())
}