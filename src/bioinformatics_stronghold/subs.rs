use std::error::Error;
use std::fs;
use bio::pattern_matching::kmp::KMP;

#[allow(dead_code)]
pub fn subs(filename :&str) -> Result<(), Box<dyn Error>> {

	let reader = fs::read_to_string(filename)?;
	let input: Vec<&str> = reader.split("\n").collect();

	let text = input[0].as_bytes();
	let pattern = input[1].as_bytes();
	let kmp = KMP::new(pattern);
	let _out:Vec<usize> = kmp.find_all(text).map(|x|{print!("{} ", x+1); x}).collect();

	Ok(())
}
