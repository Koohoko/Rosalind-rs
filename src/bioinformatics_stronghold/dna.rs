// use std::io;
use std::error::Error;

fn count_nt(input: String) -> (u32, u32, u32, u32) {
	let mut a_count: u32 = 0;
	let mut c_count: u32 = 0;
	let mut g_count: u32 = 0;
	let mut t_count: u32 = 0;
	// let mut o_count: u32 = 0;
	for nt in input.trim().chars(){
		match nt {
			'A' => a_count += 1,
			'C' => c_count += 1,
			'G' => g_count += 1,
			'T' => t_count += 1,
			// 'a' => a_count += 1,
			// 'c' => c_count += 1,
			// 'g' => g_count += 1,
			// 't' => t_count += 1,
			// _ => o_count += 1,
			_ => (),
		}
	}
	(a_count, c_count, g_count, t_count)
}

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let input = std::fs::read_to_string(file_path)?;
	let nt_count  = count_nt(input);
	println!("{:?}", nt_count);
	Ok(())
}
