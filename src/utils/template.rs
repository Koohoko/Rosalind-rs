use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn iev(filename :&str) -> Result<(), Box<dyn Error>> {
	let reader = fs::read_to_string(filename)?;
	
	Ok(())
}