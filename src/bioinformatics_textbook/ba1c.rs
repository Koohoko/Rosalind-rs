use std::error::Error;
use std::fs::read_to_string;
use crate::bioinformatics_stronghold::revc;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let input_string = read_to_string(file_path)?;
	let result  = revc::revc(& input_string);
	println!("{}", result);
	Ok(())
}