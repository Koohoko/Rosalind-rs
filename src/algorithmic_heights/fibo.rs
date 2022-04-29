// use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let n:usize = read_to_string(file_path).unwrap().trim().parse().unwrap();
	println!("{}", fibo(n));
	Ok(())
}

fn fibo(n:usize) -> usize {
	let mut tmp:Vec<usize> = vec![0, 1, 1];
	if n<=3 {
		tmp[n]
	} else {
		for _ in 4..=n {
			tmp.push(tmp[1]+tmp[2]);
			tmp.remove(0);
		}
		tmp[1]+tmp[2]
	}

}