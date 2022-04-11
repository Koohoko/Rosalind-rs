use std::error::Error;
use std::fs;
use std::iter::zip;

#[allow(dead_code)]
pub fn hamm_dist(filename :&str) -> Result<(), Box<dyn Error>> {

	let reader = fs::read_to_string(filename)?;
	let input: Vec<&str> = reader.split("\n").collect();
	let seq0 = input[0];
	let seq1 = input[1];
	
	let iter = zip(seq0.chars(), seq1.chars());
	let mut hamm_dist = 0;
	for pair in iter {
		if pair.0 != pair.1 {
			hamm_dist += 1;
		}
	}

	println!("{}", hamm_dist);
	Ok(())
}