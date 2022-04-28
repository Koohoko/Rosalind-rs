use std::error::Error;
use std::fs::read_to_string;
use itertools::Itertools;

#[allow(dead_code)]
pub fn sign(filename:&str) -> Result<(), Box<dyn Error>>{
	let input = read_to_string(filename)?;
	let mut input = input.lines();
	let n:usize = input.next().unwrap().trim().parse()?;
	let perms:Vec<Vec<usize>> = (1..=n).permutations(n).collect();
	// dbg!(perms);
	let mut results = Vec::new();

	fn give_sign(pos:usize, perm:&mut Vec<i32>, results:&mut Vec<Vec<i32>>, n:usize){
		if pos == n {
			results.push(perm.clone());
			return;
		}

		for sign in [false, true]{
			if sign {
				perm[pos] = -perm[pos]
			};
			give_sign(pos+1, perm, results, n)
		}

	}

	for perm in perms {
		let mut perm_i32:Vec<i32> = perm.iter().map(|x|*x as i32).collect();
		give_sign(0, &mut perm_i32, &mut results, n)
	}

	println!("{}", results.len());

	for rst in results {
		let mut iter = rst.iter();
		if let Some(item) = iter.next() {
			print!("{}", item);
	
			for item in iter {
				print!(" {}", item);
			}
			println!("");
		}
	}
	
	Ok(())
}