// use std::io;
use std::error::Error;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let input_string = std::fs::read_to_string(file_path)?;
	let result  = revc(& input_string);
	println!("{:?}", result);
	Ok(())
}

fn revc(input_string: & String) -> String {
	let out_string = input_string.trim().chars().rev().map(complement).collect();
	out_string
}

fn complement(nt: char) -> char {
	match nt {
		'A' => 'T',
		'C' => 'G',
		'G' => 'C',
		'T' => 'A',
		_ => panic!("non DNA character: {}", nt),
	}
}