use std::error::Error;
use std::fs::read_to_string;

pub fn lexf(filename:&str) -> Result<(), Box<dyn Error>>{
	let input = read_to_string(filename)?;
	let mut input = input.lines();
	let alphabets = input.next().unwrap();
	let mut alphabets:Vec<_> = alphabets.trim().split_whitespace().map(|x|x.chars().next().unwrap()).collect();
	alphabets.sort_by(|a, b| a.partial_cmp(b).unwrap());
	let n:usize = input.next().unwrap().trim().parse()?;

	let mut str_tmp = vec![alphabets[0]; n];

	// let i = 0_usize;
	fn recursive(pos:usize, str_tmp:&mut Vec<char>, alphabets:&Vec<char>, n:usize) {
		if pos == n {
			for item in str_tmp {
				print!("{}", item);
			}
			println!("");
			return;
		}
		
		for i in 0..alphabets.len() {
			str_tmp[pos] = alphabets[i];
			recursive(pos+1, str_tmp, alphabets, n);
		}
		
	}

	recursive(0, &mut str_tmp, &alphabets, n);
	Ok(())
}