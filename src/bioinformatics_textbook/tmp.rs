use needletail::{Sequence};
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn ba1a(filename:&str) -> Result<(), Box<dyn std::error::Error>>{
	let input = read_to_string(filename)?;
	let input = input.lines();

	

	Ok(())
}