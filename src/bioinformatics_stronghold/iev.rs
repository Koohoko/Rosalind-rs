// the number of couples having the following genotypes:
// AA-AA
// AA-Aa
// AA-aa
// Aa-Aa
// Aa-aa
// aa-aa

use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn iev(filename :&str) -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string(filename)?;
	let nums:Vec<usize> = input.trim().split(" ").map(|x| x.trim().parse().expect("please input number")).collect();

	let mut dominant_phen_offspring:f64 = nums.iter().take(3).map(|x| (x*2) as f64).sum();
	dominant_phen_offspring += (nums[3] as f64)*2.0*0.75;
	dominant_phen_offspring += (nums[4] as f64)*2.0*0.5;

	println!("{}", dominant_phen_offspring);
	Ok(())
}

