use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn lexv(filename:&str) -> Result<(), Box<dyn Error>>{
	let input = read_to_string(filename)?;
	let mut input = input.lines();
	let alphabets = input.next().unwrap();
	let alphabets:Vec<_> = alphabets.trim().split_whitespace().map(|x|x.chars().next().unwrap()).collect();
	let n:usize = input.next().unwrap().trim().parse()?;

	let mut str_tmp = Vec::new();
	let mut results = Vec::new();

	// let i = 0_usize;
	fn recursive(pos:usize, str_tmp:&mut Vec<char>, alphabets:&Vec<char>, n:usize, results:&mut Vec<Vec<char>>) {
		if pos == n {
			return;
		}
		
		for i in 0..alphabets.len() {
			str_tmp.push(alphabets[i]);
			results.push(str_tmp.clone());
			recursive(pos+1, str_tmp, alphabets, n, results);
			str_tmp.pop();
		}
	}
	
	recursive(0, &mut str_tmp, &alphabets, n, &mut results);

	for str_i in results {
		for item in str_i {
			print!("{}", item);
		}
		println!("");
	}
	

	Ok(())
}