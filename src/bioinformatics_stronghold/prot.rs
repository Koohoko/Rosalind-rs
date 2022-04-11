use protein_translate::translate; //use array indexing is faster than hashmap https://github.com/dweb0/protein-translate
use std::error::Error;
use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn prot(filename :&str) -> Result<(), Box<dyn Error>> {

	let input = fs::read_to_string(filename)?;
	let dna = input.trim().as_bytes();
	let protein = translate(dna);

	let re = Regex::new("\\*$").unwrap();
	let protein = re.replace_all(&protein, "");
	println!("{}", protein);

	Ok(())
}
