// use std::io;
use std::error::Error;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>> {
	let mut input_string = std::fs::read_to_string(file_path)?;
	let result  = rna(&mut input_string);
	println!("{:?}", result);
	Ok(())
}

fn rna(input_string: &mut String) -> String {
	let input_string = input_string.trim().replace("T", "U");
	input_string	
}